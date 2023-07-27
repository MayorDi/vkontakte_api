use json::JsonValue;

use crate::api::Api;

/// Account
/// ===
/// Добавляет пользователя или группу в черный список.
pub async fn ban() { unimplemented!() }

/// Account
/// ===
/// Позволяет сменить пароль пользователя после успешного восстановления доступа к аккаунту через СМС,
/// используя метод [`auth.restore`](https://dev.vk.com/ru/method/auth.restore).
pub async fn change_password() { unimplemented!() }

/// Account
/// ===
/// Возвращает список активных рекламных предложений (офферов), 
/// выполнив которые, пользователь сможет получить соответствующее количество голосов 
/// на свой счёт внутри приложения.
pub async fn get_active_offers() { unimplemented!() }

/// Account
/// ===
/// Метод получает настройки пользователя вашего [приложения](https://vk.com/apps?act=manage).
pub async fn get_app_permissions() { unimplemented!() }

/// Account
/// ===
/// Возвращает список пользователей, находящихся в черном списке.
pub async fn get_banned(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getBanned?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    let res = reqwest::get(req).await.expect("Could not get a response or form a request.");
    let txt = res.text().await.expect("Could not get the request text.");
    let obj = json::parse(&txt).expect("Could not convert to json");

    if obj.has_key("response") {
        return Ok(obj);
    }
    
    return Err(obj);
}

/// Account
/// ===
/// Метод возвращает ненулевые значения счётчиков пользователя.
pub async fn get_counters() { unimplemented!() }

/// Account
/// ===
/// Возвращает информацию о текущем аккаунте.
pub async fn get_info() { unimplemented!() }

/// Account
/// ===
/// Возвращает информацию о текущем профиле.
pub async fn get_profile_info() { unimplemented!() }

/// Account
/// ===
/// Позволяет получать настройки Push-уведомлений.
pub async fn get_push_settings() { unimplemented!() }

/// Account
/// ===
/// Позволяет искать пользователей ВКонтакте, 
/// используя телефонные номера, email-адреса, 
/// и идентификаторы пользователей в других сервисах. 
/// Найденные пользователи могут быть также в 
/// дальнейшем получены методом [`friends.getSuggestions`](https://dev.vk.com/ru/method/friends.getSuggestions).
pub async fn lookup_contacts() { unimplemented!() }

/// Account
/// ===
/// Подписывает устройство на базе iOS, Android, Windows Phone или Mac на получение push-уведомлений.
pub async fn register_device() { unimplemented!() }

/// Account
/// ===
/// Редактирует информацию текущего профиля.
pub async fn save_profile_info() { unimplemented!() }

/// Account
/// ===
/// Позволяет редактировать информацию о текущем аккаунте.
pub async fn set_info() { unimplemented!() }

/// Account
/// ===
/// Устанавливает короткое название приложения (до 17 символов), 
/// которое выводится пользователю в левом меню.
pub async fn set_name_in_menu() { unimplemented!() }

/// Account
/// ===
/// Помечает текущего пользователя как `offline` 
/// (только в текущем приложении).
pub async fn set_offline() { unimplemented!() }

/// Account
/// ===
/// Помечает текущего пользователя как `online` на 5 минут.
pub async fn set_online() { unimplemented!() }

/// Account
/// ===
/// Изменяет настройку push-уведомлений.
pub async fn set_push_settings() { unimplemented!() }

/// Account
/// ===
/// Отключает push-уведомления на заданный промежуток времени.
pub async fn set_silence_mode() { unimplemented!() }

/// Account
/// ===
/// Удаляет пользователя или группу из черного списка.
pub async fn unban() { unimplemented!() }

/// Account
/// ===
/// Отписывает устройство от Push уведомлений.
pub async fn unregister_device() { unimplemented!() }