use json::JsonValue;

use super::account::get_result_responce;
use super::Api;

/// Позволяет восстановить доступ к аккаунту, используя код, полученный через SMS.
/// 
/// > Этот метод можно вызвать с [ключом доступа пользователя](https://dev.vk.com/ru/api/access-token/getting-started#Ключ%20доступа%20пользователя).
/// 
/// > Этот метод можно вызвать с [сервисным ключом доступа](https://dev.vk.com/ru/api/access-token/getting-started#Сервисный%20ключ%20доступа).
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `phone` - Номер телефона пользователя;
/// * `last_name` - Фамилия пользователя;
/// * `oauth` - Список `OAuth` параметров.
pub async fn restore(
    api: &Api,
    phone: String,
    last_name: String,
    oauth: OAuthParameters,
) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/auth.restore?access_token={}&phone={phone}&last_name={last_name}&v={}",
        api.get_access_token(),
        api.get_version(),
    );

    let res = get_result_responce(req);
    
    match res.await {
        Ok(res) => {
            let sid = res["responce"]["sid"].as_str().unwrap();
            let OAuthParameters {
                grant_type,
                client_id,
                client_secret,
                username,
                scope,
                code,
            } = oauth;
            
            let req = format!("https://oauth.vk.com/token?grant_type={grant_type}&client_id={client_id}&client_secret={client_secret}&username={username}&scope={scope}&sid={sid}&code={code}");
            
            
            get_result_responce(req).await
        }

        Err(err) => Err(err)
    }
}

#[derive(Debug, Clone)]
pub struct OAuthParameters { 
    /// необходимо передать значение: restore_code;
    pub grant_type: String, 
    /// Идентификатор приложения;
    pub client_id: String, 
    /// Секретный ключ;
    pub client_secret: String, 
    /// Номер телефона по которому был восстановлен пароль;
    pub username: String, 
    /// список [прав доступа, разделенных через запятую;](https://dev.vk.com/ru/reference/access-rights)
    pub scope: String,
    /// код, полученный через SMS.
    pub code: String,
}