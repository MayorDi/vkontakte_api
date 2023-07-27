use json::JsonValue;

use crate::api::Api;

/// Добавляет пользователя или группу в черный список.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `owner_id` - Идентификатор пользователя или сообщества, 
/// которое будет добавлено в черный список.
/// Если указанный пользователь является другом текущего пользователя 
/// или имеет от него входящую или исходящую заявку в друзья, 
/// то для добавления пользователя в черный список Ваше приложение должно иметь права: `friends`.
/// 
/// # Return
/// После успешного выполнения возвращает 1. <br>
/// `{"response": 1 }`.
pub async fn ban(api: &Api, owner_id: i64) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.ban?access_token={}&owner_id={}&v={}",
        api.get_access_token(),
        owner_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Позволяет сменить пароль пользователя после успешного восстановления доступа к аккаунту через СМС,
/// используя метод [`auth.restore`](https://dev.vk.com/ru/method/auth.restore).
pub async fn change_password() { unimplemented!() }

/// Возвращает список активных рекламных предложений (офферов), 
/// выполнив которые, пользователь сможет получить соответствующее количество голосов 
/// на свой счёт внутри приложения.
pub async fn get_active_offers() { unimplemented!() }

/// Метод получает настройки пользователя вашего [приложения](https://vk.com/apps?act=manage).
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `user_id` - Идентификатор пользователя, настройки которого необходимо получить.
/// 
/// # Return
/// Функция возвращает битовую маску прав доступа (_integer_).
/// > <b>Примечание</b>. 
/// > Чтобы понять, что означает вернувшийся результат, 
/// > смотрите раздел [Список возможных настроек прав доступа](https://dev.vk.com/ru/reference/access-rights).
pub async fn get_app_permissions(api: &Api, user_id: i64) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getAppPermissions?access_token={}&user_id={}&v={}",
        api.get_access_token(),
        user_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Возвращает список пользователей, находящихся в черном списке.
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_.
/// 
pub async fn get_banned(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getBanned?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Метод возвращает ненулевые значения счётчиков пользователя.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `user_id` - Идентификатор пользователя, настройки которого необходимо получить.
/// 
/// # Return
/// Функция возвращает объект. Поля объекта:
/// ``` json
/// {
///     // Счётчик новых заявок в друзья. Поле возвращается,
///     // если передан параметр filter со значением friends.
///     "friends": integer, 
///     // Счётчик предлагаемых друзей. Поле возвращается, 
///     // если передан параметр filter со значением friends_suggestions.
///     "friends_suggestions": integer, 
///     // Счётчик новых сообщений. Поле возвращается, 
///     // если передан параметр filter со значением messages.
///     "messages": integer, 
///     // Счётчик новых отметок на фотографиях. Поле возвращается, 
///     // если передан параметр filter со значением photos.
///     "photos": integer, 
///     // Счётчик новых отметок на видеозаписях. Поле возвращается, 
///     // если передан параметр filter со значением videos.
///     "videos": integer, 
///     // Счётчик подарков. Поле возвращается, 
///     // если передан параметр filter со значением gifts.
///     "gifts": integer, 
///     // Счётчик событий. Поле возвращается, 
///     // если передан параметр filter со значением events.
///     "events": integer, 
///     // Счётчик сообществ. Поле возвращается, 
///     // если передан параметр filter со значением groups.
///     "groups": integer, 
///     // Счётчик ответов. Поле возвращается, 
///     // если передан параметр filter со значением notifications.
///     "notifications": integer, 
///     // Счётчик запросов в мобильных играх. 
///     // Поле возвращается, если передан параметр filter со значением sdk.
///     "sdk": integer, 
///     // Счётчик уведомлений от приложений. 
///     // Поле возвращается, если передан параметр filter со значением app_requests.
///     "app_requests": integer, 
///     // Счётчик рекомендаций друзей. Поле возвращается, 
///     // если передан параметр filter со значением friends_recommendations.
///     "friends_recommendations": integer, 
/// }
/// ```
pub async fn get_counters(api: &Api, user_id: i64) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getCounters?access_token={}&user_id={}&v={}",
        api.get_access_token(),
        user_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Возвращает информацию о текущем аккаунте.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `user_id` - Идентификатор пользователя, настройки которого необходимо получить.
/// 
/// # Return
/// 
/// Функция возвращает объект, содержащий следующие поля:
/// ``` json
/// {
///     // Строковой код страны, определенный по IP адресу, с которого сделан запрос.
///     "country": string, 
///     // [0, 1] Информация о том, включено ли безопасное соединение для аккаунта. 
///     // 1 — включено, 0 — не включено.
///     "https_required": integer, 
///     // [0, 1] Информация о том, включена ли двухфакторная аутентификация для аккаунта. 
///     // 1 — включена, 0 — не включена.
///     "2fa_required": integer, 
///     // [0, 1] Информация о том, показываются ли по умолчанию на стене только записи пользователя. 
///     //1 — да, 0 — нет.
///     "own_posts_default": integer, 
///     // [0, 1] Информация о том, отключено ли комментирование записей на стене. 
///     // 1 — да, 0 — нет.
///     "no_wall_replies": integer, 
///     // [0, 1] Информация о том, прошел ли пользователь обучение по использованию приложения.
///     "intro": integer, 
///     // Идентификатор текущего языка пользователя.
///     "lang": integer,
/// }
/// ```
pub async fn get_info(api: &Api, user_id: i64) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getInfo?access_token={}&user_id={}&v={}",
        api.get_access_token(),
        user_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Возвращает информацию о текущем профиле.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `user_id` - Идентификатор пользователя, настройки которого необходимо получить.
/// 
/// # Return
/// [DOC](https://dev.vk.com/ru/method/account.getProfileInfo)
pub async fn get_profile_info(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getProfileInfo?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Позволяет получать настройки Push-уведомлений.
pub async fn get_push_settings() { unimplemented!() }

/// Позволяет искать пользователей ВКонтакте, 
/// используя телефонные номера, email-адреса, 
/// и идентификаторы пользователей в других сервисах. 
/// Найденные пользователи могут быть также в 
/// дальнейшем получены методом [`friends.getSuggestions`](https://dev.vk.com/ru/method/friends.getSuggestions).
pub async fn lookup_contacts() { unimplemented!() }

/// Подписывает устройство на базе iOS, Android, Windows Phone или Mac на получение push-уведомлений.
pub async fn register_device() { unimplemented!() }

/// Редактирует информацию текущего профиля.
pub async fn save_profile_info() { unimplemented!() }

/// Позволяет редактировать информацию о текущем аккаунте.
pub async fn set_info() { unimplemented!() }

/// Устанавливает короткое название приложения (до 17 символов), 
/// которое выводится пользователю в левом меню.
pub async fn set_name_in_menu() { unimplemented!() }

/// Помечает текущего пользователя как `offline` 
/// (только в текущем приложении).
pub async fn set_offline() { unimplemented!() }

/// Помечает текущего пользователя как `online` на 5 минут.
pub async fn set_online() { unimplemented!() }

/// Изменяет настройку push-уведомлений.
pub async fn set_push_settings() { unimplemented!() }

/// Отключает push-уведомления на заданный промежуток времени.
pub async fn set_silence_mode() { unimplemented!() }

/// Удаляет пользователя или группу из черного списка.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `owner_id` - Идентификатор пользователя или группы, которого нужно удалить из черного списка.
/// 
/// # Return
/// После успешного выполнения возвращает 1. <br>
/// `{"response": 1 }`.
pub async fn unban(api: &Api, owner_id: i64) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.unban?access_token={}&owner_id={}&v={}",
        api.get_access_token(),
        owner_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Отписывает устройство от Push уведомлений.
pub async fn unregister_device() { unimplemented!() }

async fn get_result_responce(req: String) -> Result<JsonValue, JsonValue>  {
    let res = reqwest::get(req).await.expect("Could not get a response or form a request.");
    let txt = res.text().await.expect("Could not get the request text.");
    let obj = json::parse(&txt).expect("Could not convert to json");

    if obj.has_key("response") {
        return Ok(obj);
    }
    
    return Err(obj);
}