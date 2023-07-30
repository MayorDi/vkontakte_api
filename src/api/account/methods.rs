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
/// 
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `new_password` - Новый пароль, который будет установлен в качестве текущего. Обязательный параметр Мин. длина = 6;
/// 
/// ### Необязательные
/// * `restore_sid`: `String` - Идентификатор сессии, полученный при восстановлении доступа используя метод auth.restore. (В случае если пароль меняется сразу после восстановления доступа);
/// * `change_password_hash`: `String` Хэш, полученный при успешной OAuth авторизации по коду полученному по СМС (В случае если пароль меняется сразу после восстановления доступа);
/// * `old_password`: `String` - Текущий пароль пользователя.
/// 
/// 
/// # Examples
/// ``` rust
/// change_password(&api, 123, create_query! {
///     old_password: 321
/// });
/// ```
/// 
/// 
/// # Return
/// Возвращает объект с единственным полем `token`, содержащим новый токен.
pub async fn change_password(
    api: &Api, 
    new_password: &str,
    query: String
) -> Result<JsonValue, JsonValue> {

    let req = format!(
        "https://api.vk.com/method/account.changePassword?access_token={}&new_password={}&v={}{query}",
        api.get_access_token(),
        new_password,
        api.get_version()
    );

    get_result_responce(req).await
}

/// Возвращает список активных рекламных предложений (офферов), 
/// выполнив которые, пользователь сможет получить соответствующее количество голосов 
/// на свой счёт внутри приложения.
/// 
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// 
/// ### Необязательные
/// * `offset` - Смещение, необходимое для выборки определенного подмножества офферов;
/// * `count` - Количество офферов, которое необходимо получить.
/// 
/// # Examples
/// ``` rust
/// get_active_offers(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// Возвращает массив, состоящий из общего количества старгетированных на текущего пользователя 
/// специальных предложений (первый элемент), и списка объектов с информацией о предложениях. 
/// 
/// В случае, если на пользователя не старгетировано ни одного специального предложения, 
/// массив будет содержать элемент `0` (количество специальных предложений).
pub async fn get_active_offers(
    api: &Api,
    query: String,
) -> Result<JsonValue, JsonValue> {

    let req = format!(
        "https://api.vk.com/method/account.getActiveOffers?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );

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
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `id`: `usize - обязательно
/// * `first_name`: `String`
/// * `last_name`: `String`
/// * `maiden_name`: `String`
/// * `screen_name`: `String`
/// * `cancel_request_id`: `usize`
/// * `sex`: `usize`
/// * `relation`: `usize`
/// * `relation_partner_id`: `i64`
/// * `bdate`: `String`
/// * `bdate_visibility`: `usize`
/// * `home_town`: `String`
/// * `country_id`: `usize`
/// * `city_id`: `usize`
/// * `status`: `usize`
/// 
/// 
/// # Examples
/// ``` rust
/// save_profile_info(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// [DOC](https://dev.vk.com/ru/method/account.registerDevice)
pub async fn save_profile_info(api: &Api, query: String) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.saveProfileInfo?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );
    
    get_result_responce(req).await
}

/// Позволяет редактировать информацию о текущем аккаунте.
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
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
/// 
/// # Examples
/// ``` rust
/// set_info(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// В результате успешного выполнения возвращает `1`.
pub async fn set_info(api: &Api, query: String) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.setInfo?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );

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
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `device_id` - Уникальный идентификатор устройства;
/// * `settings` - Сериализованный JSON-объект, описывающий настройки уведомлений в [специальном формате](https://dev.vk.com/ru/reference/objects/push-settings);
/// * `key` - Ключ уведомления;
/// * `value` - Новое значение уведомления в [специальном формате](https://dev.vk.com/ru/reference/objects/push-settings).
/// 
/// 
/// # Examples
/// ``` rust
/// set_push_settings(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// После успешного выполнения возвращает 1.
pub async fn set_push_settings(api: &Api, query: String) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.setPushSettings?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Отключает push-уведомления на заданный промежуток времени.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
///   полученным в Standalone-приложении через [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user)
/// 
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
/// 
/// # Examples
/// ``` rust
/// set_silence_mode(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// После успешного выполнения возвращает 1.
pub async fn set_silence_mode(api: &Api, query: String) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/account.setSilenceMode?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );

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
/// 
/// # Examples
/// ``` rust
/// unregister_device(&api, create_query!{});
/// ```
/// 
/// 
/// # Return
/// После успешного выполнения возвращает 1. 
pub async fn unregister_device(api: &Api, query: String) -> Result<JsonValue, JsonValue> { 
    let req = format!(
        "https://api.vk.com/method/account.unban?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );


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