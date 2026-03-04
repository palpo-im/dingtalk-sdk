//! DingTalk Webhook/Callback handling support.

use crate::error::{Error, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackRequest {
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "encrypt")]
    pub encrypt: String,
    #[serde(rename = "msg_signature")]
    pub msg_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackResponse {
    #[serde(rename = "msg_signature")]
    pub msg_signature: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "encrypt")]
    pub encrypt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackEvent {
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: i64,
    #[serde(rename = "CorpId")]
    pub corp_id: String,
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "StaffId", skip_serializing_if = "Option::is_none")]
    pub staff_id: Option<String>,
    #[serde(rename = "AgentId", skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
    #[serde(rename = "OpenConversationId", skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(rename = "ConversationId", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "ChatId", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[serde(rename = "SessionWebhook", skip_serializing_if = "Option::is_none")]
    pub session_webhook: Option<String>,
    #[serde(
        rename = "SessionWebhookExpiredTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_webhook_expired_time: Option<i64>,
    #[serde(rename = "ProcessInstanceId", skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    #[serde(rename = "ProcessCode", skip_serializing_if = "Option::is_none")]
    pub process_code: Option<String>,
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(
        rename = "FormComponentValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub form_component_values: Option<Vec<FormField>>,
    #[serde(rename = "DeptId", skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    #[serde(rename = "Tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct CallbackCrypto {
    token: String,
    encoding_aes_key: String,
    corp_id: String,
}

impl CallbackCrypto {
    pub fn new(token: String, encoding_aes_key: String, corp_id: String) -> Result<Self> {
        let key = if encoding_aes_key.ends_with('=') {
            encoding_aes_key
        } else {
            format!("{}=", encoding_aes_key)
        };

        Ok(Self {
            token,
            encoding_aes_key: key,
            corp_id,
        })
    }

    pub fn verify_signature(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypt: &str,
    ) -> bool {
        let mut arr = vec![self.token.as_str(), timestamp, nonce, encrypt];
        arr.sort();

        let combined = arr.join("");
        let expected_signature = compute_sha1(&combined);

        expected_signature == signature
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String> {
        let encrypted_bytes = BASE64.decode(encrypted)?;
        let key = BASE64.decode(&self.encoding_aes_key)?;

        if encrypted_bytes.len() < 16 {
            return Err(Error::invalid_param("Encrypted message too short"));
        }

        let iv = &encrypted_bytes[..16];
        let encrypted_content = &encrypted_bytes[16..];
        let aes_key = &key[..32];

        let decrypted = aes_decrypt(aes_key, iv, encrypted_content)?;

        if decrypted.len() < 20 {
            return Err(Error::invalid_param("Decrypted message too short"));
        }

        let msg_len =
            u32::from_be_bytes([decrypted[16], decrypted[17], decrypted[18], decrypted[19]])
                as usize;
        if decrypted.len() < 20 + msg_len {
            return Err(Error::invalid_param("Invalid message length"));
        }

        String::from_utf8(decrypted[20..20 + msg_len].to_vec())
            .map_err(|e| Error::invalid_param(&format!("Invalid UTF-8: {}", e)))
    }

    pub fn encrypt(&self, message: &str) -> Result<String> {
        let key = BASE64.decode(&self.encoding_aes_key)?;

        let random = generate_random_string(16);
        let msg_len = message.len() as u32;
        let msg_len_bytes = msg_len.to_be_bytes();
        let corp_bytes = self.corp_id.as_bytes();

        let mut plain = Vec::new();
        plain.extend_from_slice(random.as_bytes());
        plain.extend_from_slice(&msg_len_bytes);
        plain.extend_from_slice(message.as_bytes());
        plain.extend_from_slice(corp_bytes);

        let pad_len = 32 - (plain.len() % 32);
        let pad_byte = pad_len as u8;
        for _ in 0..pad_len {
            plain.push(pad_byte);
        }

        let iv = generate_random_bytes(16);
        let encrypted = aes_encrypt(&key[..32], &iv, &plain)?;

        let mut result = Vec::new();
        result.extend_from_slice(&iv);
        result.extend_from_slice(&encrypted);

        Ok(BASE64.encode(&result))
    }

    pub fn parse_event(&self, encrypted: &str) -> Result<CallbackEvent> {
        let json = self.decrypt(encrypted)?;
        serde_json::from_str(&json).map_err(Error::from)
    }
}

fn compute_sha1(input: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn aes_encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
    type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

    let cipher = Aes256CbcEnc::new(key.into(), iv.into());
    let ct_len = data.len() + 16 - (data.len() % 16);
    let mut buf = vec![0u8; ct_len];
    buf[..data.len()].copy_from_slice(data);
    let encrypted_len: usize = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buf, data.len())
        .map_err(|e| Error::invalid_param(&format!("Encryption error: {}", e)))?
        .len();
    Ok(buf[..encrypted_len].to_vec())
}

fn aes_decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    let cipher = Aes256CbcDec::new(key.into(), iv.into());
    let mut buf = data.to_vec();
    let decrypted_len: usize = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| Error::invalid_param(&format!("Decryption error: {}", e)))?
        .len();
    Ok(buf[..decrypted_len].to_vec())
}

fn generate_random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn generate_random_bytes(len: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen::<u8>()).collect()
}

pub struct CallbackHandler {
    crypto: CallbackCrypto,
}

impl CallbackHandler {
    pub fn new(token: String, encoding_aes_key: String, corp_id: String) -> Result<Self> {
        Ok(Self {
            crypto: CallbackCrypto::new(token, encoding_aes_key, corp_id)?,
        })
    }

    pub fn verify_url(
        &self,
        signature: &str,
        timestamp: &str,
        nonce: &str,
        echo_str: &str,
    ) -> Result<String> {
        if !self
            .crypto
            .verify_signature(signature, timestamp, nonce, echo_str)
        {
            return Err(Error::auth_error("Invalid signature"));
        }

        self.crypto.decrypt(echo_str)
    }

    pub fn handle_callback(&self, request: &CallbackRequest) -> Result<CallbackEvent> {
        if !self.crypto.verify_signature(
            &request.msg_signature,
            &request.timestamp,
            &request.nonce,
            &request.encrypt,
        ) {
            return Err(Error::auth_error("Invalid signature"));
        }

        self.crypto.parse_event(&request.encrypt)
    }

    pub fn create_response(
        &self,
        message: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<CallbackResponse> {
        let encrypted = self.crypto.encrypt(message)?;

        let mut arr = vec![self.crypto.token.as_str(), timestamp, nonce, &encrypted];
        arr.sort();
        let signature = compute_sha1(&arr.join(""));

        Ok(CallbackResponse {
            msg_signature: signature,
            timestamp: timestamp.to_string(),
            nonce: nonce.to_string(),
            encrypt: encrypted,
        })
    }
}

pub mod event_types {
    pub const USER_ADD_ORG: &str = "user_add_org";
    pub const USER_MODIFY_ORG: &str = "user_modify_org";
    pub const USER_LEAVE_ORG: &str = "user_leave_org";
    pub const USER_ACTIVE_ORG: &str = "user_active_org";
    pub const ORG_ADMIN_REMOVE: &str = "org_admin_remove";
    pub const ORG_ADMIN_ADD: &str = "org_admin_add";
    pub const ORG_CHANGE: &str = "org_change";
    pub const ORG_DEPT_CREATE: &str = "org_dept_create";
    pub const ORG_DEPT_MODIFY: &str = "org_dept_modify";
    pub const ORG_DEPT_REMOVE: &str = "org_dept_remove";
    pub const CHAT_ADD_MEMBER: &str = "chat_add_member";
    pub const CHAT_REMOVE_MEMBER: &str = "chat_remove_member";
    pub const CHAT_QUIT: &str = "chat_quit";
    pub const CHAT_DISBAND: &str = "chat_disband";
    pub const CHAT_UPDATE_OWNER: &str = "chat_update_owner";
    pub const CHAT_UPDATE_TITLE: &str = "chat_update_title";
    pub const CHAT_UPDATE_ICON: &str = "chat_update_icon";
    pub const MESSAGE_READ: &str = "message_read";
    pub const LABEL_CONF: &str = "label_conf";
    pub const LABEL_USER_CHANGE: &str = "label_user_change";
    pub const BPM_INSTANCE_CHANGE: &str = "bpm_instance_change";
    pub const BPM_TASK_CHANGE: &str = "bpm_task_change";
    pub const CHECK_IN: &str = "check_in";
    pub const ATTENDANCE_CHECK: &str = "attendance_check";
    pub const ATTENDANCE_SCHEDULE_CHANGE: &str = "attendance_schedule_change";
    pub const ATTENDANCE_OVERTIME_DURATION: &str = "attendance_overtime_duration";
    pub const ROBOT_INCOMING_MESSAGE: &str = "robot_incoming_message";
    pub const ROBOT_OUTGOING_MESSAGE: &str = "robot_outgoing_message";
}
