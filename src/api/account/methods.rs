use json::JsonValue;

use crate::api::Api;

use super::User;

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
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `new_password` - Новый пароль, который будет установлен в качестве текущего. Обязательный параметр Мин. длина = 6;
/// ### Необязательные
/// 
/// * `restore_sid` - Идентификатор сессии, полученный при восстановлении доступа используя метод auth.restore. (В случае если пароль меняется сразу после восстановления доступа);
/// * `change_password_hash` Хэш, полученный при успешной OAuth авторизации по коду полученному по СМС (В случае если пароль меняется сразу после восстановления доступа);
/// * `old_password` - Текущий пароль пользователя.
/// # Return
/// Возвращает объект с единственным полем `token`, содержащим новый токен.
pub async fn change_password(
    api: &Api, 
    new_password: &str,
    old_password: Option<&str>,
    restore_sid: Option<&str>,
    change_password_hash: Option<&str>,
) -> Result<JsonValue, JsonValue> {

    let mut req = format!(
        "https://api.vk.com/method/account.changePassword?access_token={}&new_password={}&v={}",
        api.get_access_token(),
        new_password,
        api.get_version()
    );

    if let Some(old_password) = old_password {
        req.push_str(format!("&old_password={}", old_password).as_str());
    }

    if let Some(restore_sid) = restore_sid {
        req.push_str(format!("&restore_sid={}", restore_sid).as_str());
    }

    if let Some(change_password_hash) = change_password_hash {
        req.push_str(format!("&change_password_hash={}", change_password_hash).as_str());
    }

    get_result_responce(req).await
}

/// Возвращает список активных рекламных предложений (офферов), 
/// выполнив которые, пользователь сможет получить соответствующее количество голосов 
/// на свой счёт внутри приложения.
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// 
/// ### Необязательные
/// * `offset` - Смещение, необходимое для выборки определенного подмножества офферов;
/// * `count` - Количество офферов, которое необходимо получить.
/// 
/// # Return
/// Возвращает массив, состоящий из общего количества старгетированных на текущего пользователя 
/// специальных предложений (первый элемент), и списка объектов с информацией о предложениях. 
/// <br>
/// <br>
/// В случае, если на пользователя не старгетировано ни одного специального предложения, 
/// массив будет содержать элемент `0` (количество специальных предложений).
pub async fn get_active_offers(
    api: &Api,
    offset: Option<usize>,
    count: Option<usize>,
) -> Result<JsonValue, JsonValue> {

    let mut req = format!(
        "https://api.vk.com/method/account.getActiveOffers?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(offset) = offset {
        req.push_str(format!("&offset={}", offset).as_str());
    }

    if let Some(count) = count {
        req.push_str(format!("&restore_sid={}", count).as_str());
    }

    get_result_responce(req).await
}

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
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// 
/// # Return
/// 
/// Возвращает объект, содержащий поля:
/// ``` json
/// {
///     // отключены ли уведомления
///     "disabled": ?,
///     // unixtime-значение времени, до которого временно отключены уведомления.
///     "disabled_until": ?, 
///     // список, содержащий настройки конкретных диалогов, и их количество первым элементом.
///     "conversations": ?, 
///     // объект с настройками Push-уведомлений в специальном формате.
///     "settings": ?, 
/// }
/// ```
pub async fn get_push_settings(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.getPushSettings?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Подписывает устройство на базе iOS, Android, Windows Phone или Mac на получение push-уведомлений.
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `token` - Идентификатор устройства, используемый для отправки уведомлений. (для mpns идентификатор должен представлять из себя URL для отправки уведомлений);
/// * `device_id` - Уникальный идентификатор устройства.
/// 
/// # Return
/// [DOC](https://dev.vk.com/ru/method/account.registerDevice)
pub async fn register_device(
    api: &Api, 
    token: &str, 
    device_id: &str
) -> Result<JsonValue, JsonValue> {

    let req = format!(
        "https://api.vk.com/method/account.registerDevice?access_token={}&token={}&device_id={}&v={}",
        api.get_access_token(),
        token,
        device_id,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Редактирует информацию текущего профиля.
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `user` - новые значения для старой информации.
/// 
/// # Return
/// [DOC](https://dev.vk.com/ru/method/account.registerDevice)
pub async fn save_profile_info(
    api: &Api, 
    user: &User,
) -> Result<JsonValue, JsonValue> {

    let mut req = format!(
        "https://api.vk.com/method/account.saveProfileInfo?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(first_name) = &user.first_name {
        req.push_str(format!("&first_name={}", first_name).as_str());
    }
    
    if let Some(last_name) = &user.last_name {
        req.push_str(format!("&last_name={}", last_name).as_str());
    }
    
    if let Some(maiden_name) = &user.maiden_name {
        req.push_str(format!("&maiden_name={}", maiden_name).as_str());
    }
    
    if let Some(screen_name) = &user.screen_name {
        req.push_str(format!("&screen_name={}", screen_name).as_str());
    }
    
    if let Some(cancel_request_id) = &user.cancel_request_id {
        req.push_str(format!("&cancel_request_id={}", cancel_request_id).as_str());
    }
    
    if let Some(sex) = &user.sex {
        req.push_str(format!("&sex={}", sex).as_str());
    }
    
    if let Some(relation) = &user.relation {
        req.push_str(format!("&relation={}", relation).as_str());
    }
    
    if let Some(relation_partner_id) = &user.relation_partner_id {
        req.push_str(format!("&relation_partner_id={}", relation_partner_id).as_str());
    }
    
    if let Some(bdate) = &user.bdate {
        req.push_str(format!("&bdate={}", bdate).as_str());
    }
    
    if let Some(bdate_visibility) = &user.bdate_visibility {
        req.push_str(format!("&bdate_visibility={}", bdate_visibility).as_str());
    }
    
    if let Some(home_town) = &user.home_town {
        req.push_str(format!("&home_town={}", home_town).as_str());
    }
    
    if let Some(country_id) = &user.country_id {
        req.push_str(format!("&country_id={}", country_id).as_str());
    }
    
    if let Some(city_id) = &user.city_id {
        req.push_str(format!("&city_id={}", city_id).as_str());
    }
    
    if let Some(status) = &user.status {
        req.push_str(format!("&status={}", status).as_str());
    }
    

    get_result_responce(req).await
}

/// Позволяет редактировать информацию о текущем аккаунте.
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `intro` - Битовая маска, отвечающая за прохождение обучения в мобильных клиентах.
/// * `own_posts_default` - 1 – на стене пользователя по умолчанию должны отображаться только 
///     собственные записи; 0 – на стене пользователя должны отображаться все записи.
/// * `no_wall_replies` - 1 – отключить комментирование записей на стене; 0 – разрешить комментирование.
/// * `name` - Имя настройки
/// * `value` - Значение настройки
/// 
/// # Return
/// В результате успешного выполнения возвращает `1`.
pub async fn set_info(
    api: &Api,
    intro: Option<usize>,
    own_posts_default: Option<u8>,
    no_wall_replies: Option<u8>,
    name: Option<String>,
    value: Option<String>,
) -> Result<JsonValue, JsonValue> {
    let mut req = format!(
        "https://api.vk.com/method/account.setInfo?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(intro) = intro {
        req.push_str(format!("&intro={}", intro).as_str());
    }

    if let Some(own_posts_default) = own_posts_default {
        req.push_str(format!("&own_posts_default={}", own_posts_default).as_str());
    }

    if let Some(no_wall_replies) = no_wall_replies {
        req.push_str(format!("&no_wall_replies={}", no_wall_replies).as_str());
    }

    if let Some(name) = name {
        req.push_str(format!("&name={}", name).as_str());
    }

    if let Some(value) = value {
        req.push_str(format!("&value={}", value).as_str());
    }

    get_result_responce(req).await
}

/// Помечает текущего пользователя как `offline` 
/// (только в текущем приложении).
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// 
/// # Return
/// После успешного выполнения возвращает 1. 
pub async fn set_offline(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.setOffline?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Помечает текущего пользователя как `online` на 5 минут.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// 
/// # Return
/// После успешного выполнения возвращает 1. 
pub async fn set_online(api: &Api) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.setOnline?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Изменяет настройку push-уведомлений.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `device_id` - Уникальный идентификатор устройства;
/// * `settings` - Сериализованный JSON-объект, описывающий настройки уведомлений в [специальном формате](https://dev.vk.com/ru/reference/objects/push-settings);
/// * `key` - Ключ уведомления;
/// * `value` - Новое значение уведомления в [специальном формате](https://dev.vk.com/ru/reference/objects/push-settings).
/// 
/// # Return
/// После успешного выполнения возвращает 1.
pub async fn set_push_settings(
    api: &Api, 
    device_id: Option<String>,
    settings: Option<String>,
    key: Option<String>,
    value: Option<String>,
) -> Result<JsonValue, JsonValue> {

    let mut req = format!(
        "https://api.vk.com/method/account.setPushSettings?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(device_id) = device_id {
        req.push_str(format!("&device_id={}", device_id).as_str());
    }

    if let Some(settings) = settings {
        req.push_str(format!("&settings={}", settings).as_str());
    }

    if let Some(key) = key {
        req.push_str(format!("&key={}", key).as_str());
    }

    if let Some(value) = value {
        req.push_str(format!("&value={}", value).as_str());
    }


    get_result_responce(req).await
}

/// Отключает push-уведомления на заданный промежуток времени.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `token` - Идентификатор устройства для сервиса push уведомлений.
/// * `device_id` - Уникальный идентификатор устройства.
/// * `time` - Время в секундах на которое требуется отключить уведомления, `-1` отключить навсегда.
/// * `chat_id` - Идентификатор чата, для которого следует отключить уведомления.
/// * `user_id` - Идентификатор пользователя, для которого следует отключить уведомления.
/// * `peer_id` - Идентификатор назначения. Для пользователя: `id` пользователя.
///     Для групповой беседы: `2000000000` + `id` беседы.
///     Для сообщества: `-id` сообщества.
/// * `sound` - 
///     `1` — включить звук в этом диалоге, 
///     `0` — отключить звук (параметр работает, только если в `peer_id` передан идентификатор 
///         групповой беседы или пользователя).
/// 
/// # Return
/// После успешного выполнения возвращает 1.
pub async fn set_silence_mode(
    api: &Api,
    token: Option<String>,
    device_id: Option<String>,
    time: Option<i64>,
    chat_id: Option<i64>,
    user_id: Option<i64>,
    peer_id: Option<i64>,
    sound: Option<i64>,
) -> Result<JsonValue, JsonValue> {

    let mut req = format!(
        "https://api.vk.com/method/account.setSilenceMode?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(token) = token {
        req.push_str(format!("&token={}", token).as_str());
    }

    if let Some(device_id) = device_id {
        req.push_str(format!("&device_id={}", device_id).as_str());
    }

    if let Some(time) = time {
        req.push_str(format!("&time={}", time).as_str());
    }

    if let Some(chat_id) = chat_id {
        req.push_str(format!("&chat_id={}", chat_id).as_str());
    }

    if let Some(user_id) = user_id {
        req.push_str(format!("&user_id={}", user_id).as_str());
    }

    if let Some(peer_id) = peer_id {
        req.push_str(format!("&peer_id={}", peer_id).as_str());
    }

    if let Some(sound) = sound {
        req.push_str(format!("&sound={}", sound).as_str());
    }



    get_result_responce(req).await
}

/// Удаляет пользователя или группу из черного списка.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `owner_id` - Идентификатор пользователя или группы, которого нужно удалить из черного списка.
/// 
/// # Return
/// После успешного выполнения возвращает 1.
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
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `token` - Идентификатор устройства, используемый для отправки уведомлений.
/// * `device_id` - Уникальный идентификатор устройства.
/// * `sandbox` - Флаг предназначен для iOS устройств. Возможные значения:
/// * * 1 — отписать устройство, использующего sandbox сервер для отправки push-уведомлений;
/// * * 0 — отписать устройство, не использующее sandbox сервер.
/// 
/// # Return
/// После успешного выполнения возвращает 1. 
pub async fn unregister_device(
    api: &Api,
    token: Option<String>,
    device: Option<String>,
    sandbox: Option<u8>,
) -> Result<JsonValue, JsonValue> { 
    let mut req = format!(
        "https://api.vk.com/method/account.unban?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );

    if let Some(token) = token {
        req.push_str(format!("&token={}", token).as_str());
    }

    if let Some(device) = device {
        req.push_str(format!("&device={}", device).as_str());
    }

    if let Some(sandbox) = sandbox {
        req.push_str(format!("&sandbox={}", sandbox).as_str());
    }


    get_result_responce(req).await
}

pub(crate) async fn get_result_responce(req: String) -> Result<JsonValue, JsonValue>  {
    let res = reqwest::get(req).await.expect("Could not get a response or form a request.");
    let txt = res.text().await.expect("Could not get the request text.");
    let obj = json::parse(&txt).expect("Could not convert to json");

    if obj.has_key("response") {
        return Ok(obj);
    }
    
    return Err(obj);
}