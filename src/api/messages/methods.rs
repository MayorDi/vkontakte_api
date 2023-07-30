use json::JsonValue;

use crate::api::{Api, account::get_result_responce};

/// Добавляет в мультидиалог нового пользователя.
pub async fn add_chat_user() { unimplemented!() }

/// Позволяет разрешить отправку сообщений от сообщества текущему пользователю.
pub async fn allow_messages_from_group() { unimplemented!() }

/// Создаёт беседу с несколькими участниками.
pub async fn create_chat() { unimplemented!() }

/// Удаляет сообщение.
pub async fn delete() { unimplemented!() }

/// Позволяет удалить фотографию мультидиалога.
pub async fn delete_chat_photo() { unimplemented!() }

/// Удаляет беседу.
pub async fn delete_conversation() { unimplemented!() }

/// Удаление ранее поставленной реакции
pub async fn delete_reaction() { unimplemented!() }

/// Позволяет запретить отправку сообщений от сообщества текущему пользователю.
pub async fn deny_messages_from_group() { unimplemented!() }

/// Редактирует сообщение.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
/// полученным в Standalone-приложении через 
/// [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user).
/// Требуются 
/// [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// > Этот метод можно вызвать с 
/// [ключом доступа сообщества](https://dev.vk.com/ru/api/access-token/getting-started#Ключ%20доступа%20сообщества).
/// Требуются [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `parameters` - отдельная структура с полями для разгрузки функции:
/// 
/// 
/// # Return
/// 
/// После успешного выполнения возвращает `1`.
/// 
/// 
/// # Error's Code
/// 
/// |Code   |Error                                                              |
/// |-------|-------------------------------------------------------------------|
/// |`901`  | Can't send messages for users without permission                  |
/// |`909`  | Can't edit this message, because it's too old                     |
/// |`910`  | Can't sent this message, because it's too big                     |
/// |`911`  | Keyboard format is invalid                                        |
/// |`912`  | This is a chat bot feature, change this status in settings        |
/// |`914`  | Message is too long                                               |
/// |`917`  | You don't have access to this chat                                |
/// |`920`  | Can't edit this kind of message                                   |
/// |`940`  | Too many posts in messages                                        |
/// |`946`  | Chat not supported                                                |
/// |`949`  | Can't send message, reply timed out                               |
/// 
/// В ходе выполнения могут произойти [общие ошибки](https://dev.vk.com/ru/reference/errors).
pub async fn edit(
    api: &Api,
    parameters: EditParameters,
) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/messages.edit?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );
    
    
    get_result_responce(req).await
}

/// Хранит основные параметры для функции `messages::edit(...)`.
#[derive(Debug, Default, Clone)]
pub struct EditParameters {
    /// Идентификатор назначения.
    /// 
    /// Для пользователя:
    /// * `id` пользователя.
    /// Для групповой беседы:
    /// * `2000000000` + `id` беседы.
    /// Для сообщества:
    /// * `-id` сообщества.
    /// 
    /// Обязательный параметр
    pub peer_id: i64,

    /// Текст сообщения. Обязательный параметр, если не задан параметр `attachment`.
    /// 
    /// Макс. длина = `9000`
    pub message: String,
    
    
    /// Географическая широта (от `-90` до `90`).
    pub lat: Option<String>,
    
    
    /// Географическая долгота (от `-180` до `180`).
    pub long: Option<String>,
    
    
    
    /// Медиавложения к личному сообщению, перечисленные через запятую. 
    /// Каждое прикрепление представлено в формате: `<type><owner_id>_<media_id>`
    /// `<type>` — тип медиавложения:
    /// 
    /// * `photo` — фотография;
    /// * `video` — видеозапись;
    /// * `audio` — аудиозапись;
    /// * `doc` — документ;
    /// * `wall` — запись на стене;
    /// * `market` — товар.
    /// 
    /// `<owner_id>` — идентификатор владельца медиавложения 
    /// (обратите внимание, если объект находится в сообществе, этот параметр должен быть отрицательным). 
    /// `<media_id>` — идентификатор медиавложения.
    /// 
    /// Например:
    /// 
    /// > `photo100172_166443618`
    /// 
    /// Параметр является обязательным, если не задан параметр `message`.
    /// 
    /// В случае, если прикрепляется объект, принадлежащий другому пользователю следует добавлять к 
    /// вложению его [`access_key`](https://dev.vk.com/ru/reference/objects)
    /// в формате `<type><owner_id>_<media_id>_<access_key>`, 
    /// 
    /// Например:
    /// > video85635407_165186811_69dff3de4372ae9b6e
    pub attachment: Option<String>,
    
    
    /// `1`, чтобы сохранить прикреплённые пересланные сообщения.
    pub keep_forward_messages: Option<u8>,
    
    
    /// `1`, чтобы сохранить прикреплённые внешние ссылки (сниппеты).
    pub keep_snippets: Option<u8>,
    
    
    /// Идентификатор сообщества (для сообщений сообщества с ключом доступа пользователя).
    pub group_id: Option<i64>,
    
    
    /// `1` — не создавать сниппет ссылки из сообщения.
    pub dont_parse_links: Option<u8>,
    
    
    /// `1` — отключить уведомление об упоминании в сообщении.
    pub disable_mentions: Option<u8>,
    
    
    /// Идентификатор сообщения.
    pub message_id: usize,
    
    
    /// Идентификатор сообщения в беседе.
    pub conversation_message_id: Option<usize>,
    
    
    /// Объект, описывающий [шаблоны сообщений](https://dev.vk.com/ru/api/bots/development/messages).
    pub template: Option<String>,
    
    
    /// Объект, описывающий [клавиатуру бота](https://dev.vk.com/ru/api/bots/development/keyboard).
    pub keyboard: Option<String>,
}

/// Изменяет название беседы.
pub async fn edit_chat() { unimplemented!() }

/// Метод используется для принудительного завершения звонка
pub async fn force_call_finish() { unimplemented!() }

/// Возвращает список входящих личных сообщений текущего пользователя или сообщества.
pub async fn get() { unimplemented!() }

/// Возвращает сообщения по conversation_message_id.
pub async fn get_by_conversation_message_id() { unimplemented!() }

/// Возвращает сообщения по их идентификаторам.
pub async fn get_by_id() { unimplemented!() }

/// Возвращает информацию о беседе.
pub async fn get_chat() { unimplemented!() }

/// Получает данные для превью чата с приглашением по ссылке.
pub async fn get_chat_preview() { unimplemented!() }

/// Позволяет получить список пользователей мультидиалога по его `id`.
pub async fn get_chat_users() { unimplemented!() }

/// Метод получает список участников беседы.
pub async fn get_conversation_members() { unimplemented!() }

/// Возвращает список бесед пользователя.
pub async fn get_conversations() { unimplemented!() }

/// Позволяет получить беседу по её идентификатору.
pub async fn get_conversations_by_id() { unimplemented!() }

/// Возвращает список диалогов текущего пользователя или сообщества. Актуальный метод: 
/// [`messages.getConversations`](https://dev.vk.com/ru/method/messages.getConversations).
pub async fn get_dialogs() { unimplemented!() }

/// Возвращает историю сообщений для указанного диалога.
pub async fn get_history() { unimplemented!() }

/// Возвращает материалы диалога или беседы.
pub async fn get_history_attachments() { unimplemented!() }

/// Возвращает список важных сообщений пользователя.
pub async fn get_important_messages() { unimplemented!() }

/// Метод отдает пользователей, которые подписались на определенные интенты. 
/// https://dev.vk.com/api/bots/overview
pub async fn get_intent_users() { unimplemented!() }

/// Получает ссылку для приглашения пользователя в беседу.
pub async fn get_invite_link() { unimplemented!() }

/// Метод получает текущий статус и дату последней активности пользователя.
pub async fn get_last_activity() { unimplemented!() }

/// Возвращает обновления в личных сообщениях пользователя.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
/// полученным в Standalone-приложении через 
/// [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user).
/// Требуются 
/// [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// > Этот метод можно вызвать с 
/// [ключом доступа сообщества](https://dev.vk.com/ru/api/access-token/getting-started#Ключ%20доступа%20сообщества).
/// Требуются [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `parameters` - отдельная структура с полями для разгрузки функции:
/// * * `ts`;
/// * * `pts`;
/// * * `preview_length`;
/// * * `onlines`;
/// * * `fields`;
/// * * `events_limit`;
/// * * `msgs_limit`;
/// * * `max_msg_id`;
/// * * `group_id`;
/// * * `lp_version`;
/// * * `last_n`;
/// * * `credentials`;
/// * * `extended`.
/// 
/// 
/// # Return
/// 
/// Возвращает объект, который содержит поля `history`, `messages`, 
/// а также `groups` — массив [объектов сообществ](https://dev.vk.com/ru/reference/objects/group) 
/// и `profiles` — массив [объектов пользователей](https://dev.vk.com/ru/reference/objects/user). 
/// Поле `history` представляет из себя массив, аналогичный полю `updates`, 
/// получаемому от `Long Poll` сервера, за некоторыми исключениями: для событий с кодом `4` 
/// (добавление нового сообщения) отсутствуют все поля, кроме первых трёх, 
/// а также отсутствуют события с кодами `8`, `9` (друг появился/пропал из сети) и `61`, `62` 
/// (набор текста в диалоге/беседе). 
/// Поле `messages` представляет из себя массив личных сообщений – объектов `message`, 
/// которые встречались среди событий с кодом `4` (добавление нового сообщения) из поля `history`. 
/// Каждый объект `message` содержит набор полей, описание которых доступно 
/// [здесь](https://dev.vk.com/ru/reference/objects/message). 
/// Первый элемент массива представляет собой общее количество сообщений.
/// 
/// 
/// # Limits
/// 
/// В случае, если `ts` слишком старый (больше суток), 
/// а `max_msg_id` не передан, метод может вернуть ошибку `10` (Internal server error).
/// 
/// Если количество событий превышает значение `events_limit` или 
/// количество событий с сообщениями превышает значение `msgs_limit`, 
/// ответ содержит дополнительное поле more со значением `1` — это означает, 
/// что нужно запросить оставшиеся данные с помощью запроса с параметром `max_msg_id`. 
/// Обратите внимание, что параметры `events_limit` и `msgs_limit` применяются совместно — 
/// число объектов в результате не превышает значения меньшего из этих параметров.
/// 
/// Ошибки с кодами `907` и `908` означают, что нужно получить новое значение `pts` (`ts`) и 
/// вызвать метод повторно с новыми значениями, поскольку данных для переданных значений не существует.
/// 
/// 
/// # Error's Code
/// 
/// |Code   |Error                                              |
/// |-------|---------------------------------------------------|
/// |`36`   |Method execution was interrupted due to timeout    |
/// |`907`  |Value of ts or pts is too old                      |
/// |`908`  |Value of ts or pts is too new                      |
/// |`927`  |Chat does not exist                                |
/// 
/// В ходе выполнения могут произойти [общие ошибки](https://dev.vk.com/ru/reference/errors).
pub async fn get_long_poll_history(
    api: &Api,
    parameters: LongPollHistoryParameters,
) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/messages.getLongPollHistory?access_token={}&v={}",
        api.get_access_token(),
        api.get_version()
    );
    
    
    get_result_responce(req).await
}

/// Хранит основные параметры для функции `messages::get_long_poll_history(...)`.
pub struct LongPollHistoryParameters {
    /// Последнее значение параметра ts, полученное от Long Poll сервера или с помощью метода messages.getLongPollServer.
    pub ts: usize,
    
    /// Последнее значение параметра new_pts, полученное от Long Poll сервера, используется для получения действий, которые хранятся всегда.
    pub pts: usize,
    
    /// Количество символов, по которому нужно обрезать сообщение. Укажите 0, если вы не хотите обрезать сообщение. (по умолчанию сообщения не обрезаются).
    pub preview_length: usize,
    
    /// `1` — возвращать в числе прочих события `8` и `9` (пользователь стал онлайн/оффлайн). 
    /// Учитывается только при использовании `ts`.
    pub onlines: u8,
    
    
    /// Список дополнительных полей профилей, которые необходимо вернуть. 
    /// См. подробное описание. 
    /// Доступные значения:
    /// * `about`;
    /// * `activities`;
    /// * `bdate`;
    /// * `books`;
    /// * `can_post`;
    /// * `can_see_all_posts`;
    /// * `can_see_audio`;
    /// * `can_write_private_message`;
    /// * `career`;
    /// * `city`;
    /// * `common_count`;
    /// * `connections`;
    /// * `contacts`;
    /// * `counters`;
    /// * `country`;
    /// * `domain`;
    /// * `education`;
    /// * `friend_status`;
    /// * `games`;
    /// * `has_mobile`;
    /// * `interests`;
    /// * `last_seen`;
    /// * `maiden_name`;
    /// * `military`;
    /// * `movies`;
    /// * `music`;
    /// * `occupation`;
    /// * `online`;
    /// * `online_mobile`;
    /// * `personal`;
    /// * `photo_100`;
    /// * `photo_200`;
    /// * `photo_200_orig`;
    /// * `photo_400_orig`;
    /// * `photo_50`;
    /// * `photo_id`;
    /// * `photo_max`;
    /// * `photo_max_orig`;
    /// * `quotes`;
    /// * `relation`;
    /// * `relatives`;
    /// * `schools`;
    /// * `screen_name`;
    /// * `sex`;
    /// * `site`;
    /// * `status`;
    /// * `timezone`;
    /// * `tv`;
    /// * `universities`.
    pub fields: String,

    /// Лимит по количеству всех событий в истории. 
    /// Обратите внимание, параметры `events_limit` и `msgs_limit` применяются совместно. 
    /// Число результатов в ответе ограничивается первым достигнутым лимитом.
    pub events_limit: usize,
    
    /// Лимит по количеству событий с сообщениями в истории. 
    /// Обратите внимание, параметры `events_limit` и `msgs_limit` применяются совместно. 
    /// Число результатов в ответе ограничивается первым достигнутым лимитом.
    pub msgs_limit: usize,
    
    /// Максимальный идентификатор сообщения среди уже имеющихся в локальной копии. 
    /// Необходимо учитывать как сообщения, полученные через методы `API` 
    /// (например `messages.getDialogs`, `messages.getHistory`), так и данные, 
    /// полученные из `Long Poll` сервера (события с кодом `4`).
    pub max_msg_id: usize,
    
    /// Идентификатор сообщества (для сообщений сообщества с ключом доступа пользователя).
    pub group_id: i64,
    
    /// Версия `Long Poll`.
    pub lp_version: usize,

    pub last_n: usize,
    pub credentials: u8,
    pub extended: u8,
}

/// Возвращает данные, необходимые для 
/// [подключения к Long Poll серверу](https://dev.vk.com/ru/api/user-long-poll/getting-started).
pub async fn get_long_poll_server() { unimplemented!() }

/// Получить актуальные счётчики реакций на сообщения
pub async fn get_messages_reactions() { unimplemented!() }

/// Получить список пользователей и сообществ, которые поставили реакцию на сообщение
pub async fn get_reacted_peers() { unimplemented!() }

/// Получение ассетов реакций
pub async fn get_reactions_assets() { unimplemented!() }

/// Возвращает информацию о том, разрешена ли отправка сообщений от сообщества пользователю.
pub async fn is_messages_from_group_allowed() { unimplemented!() }

/// Позволяет присоединиться к чату по ссылке-приглашению.
pub async fn join_chat_by_invite_link() { unimplemented!() }

/// Помечает беседу как отвеченную либо снимает отметку.
pub async fn mark_as_answered_conversation() { unimplemented!() }

/// Помечает сообщения как важные либо снимает отметку.
pub async fn mark_as_important() { unimplemented!() }

/// Помечает беседу как важную либо снимает отметку.
pub async fn mark_as_important_conversation() { unimplemented!() }

/// Метод помечает сообщения как прочитанные.
pub async fn mark_as_read() { unimplemented!() }

/// Закрепляет сообщение.
pub async fn pin() { unimplemented!() }

/// Исключает из мультидиалога пользователя, если текущий пользователь 
/// или сообщество является администратором беседы либо текущий пользователь 
/// пригласил исключаемого пользователя.
pub async fn remove_chat_user() { unimplemented!() }

/// Восстанавливает удаленное сообщение.
pub async fn restore() { unimplemented!() }

/// Возвращает список найденных личных сообщений текущего пользователя по введенной строке поиска.
pub async fn search() { unimplemented!() }

/// Позволяет искать диалоги.
pub async fn search_conversations() { unimplemented!() }

/// Возвращает список найденных диалогов текущего пользователя по введенной строке поиска.
pub async fn search_dialogs() { unimplemented!() }

/// Метод отправляет сообщение.
/// 
/// > Этот метод можно вызвать с ключом доступа пользователя, 
/// полученным в Standalone-приложении через 
/// [Implicit Flow](https://dev.vk.com/ru/api/access-token/implicit-flow-user).
/// Требуются 
/// [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// > Этот метод можно вызвать с 
/// [ключом доступа сообщества](https://dev.vk.com/ru/api/access-token/getting-started#Ключ%20доступа%20сообщества).
/// Требуются [права доступа](https://dev.vk.com/ru/reference/access-rights): `messages`.
/// 
/// # Arguments
/// 
/// * `api` - ссылка на _Api_ объект из которого потом будет получен _access token_ и API _version_;
/// * `parameters` - отдельная структура с полями для разгрузки функции:
/// * * `user_id`;
/// * * `random_id`;
/// * * `peer_id`;
/// * * `peer_ids`;
/// * * `domain`;
/// * * `chat_id`;
/// * * `user_ids`;
/// * * `message`;
/// * * `guid`;
/// * * `lat`;
/// * * `long`;
/// * * `attachment`;
/// * * `reply_to`;
/// * * `forward_messages`;
/// * * `forward`;
/// * * `sticker_id`;
/// * * `group_id`;
/// * * `keyboard`;
/// * * `template`;
/// * * `payload`;
/// * * `content_source`;
/// * * `dont_parse_links`;
/// * * `disable_mentions`;
/// * * `intent`;
/// * * `subscribe_id`.
/// 
/// # Return
/// 
/// Метод возвращает идентификатор отправленного сообщения. 
/// Если передан параметр `peer_ids`, метод возвращает массив объектов. 
/// Поля объекта:
/// 
/// |field                      |type       |description                                                        |
/// |---------------------------|-----------|-------------------------------------------------------------------|
/// |`peer_id`                  |`integer`  |Идентификатор назначения.                                          |
/// |`message_id`               |`integer`  |Идентификатор сообщения.                                           |
/// |`conversation_message_id`  |`integer`  |Идентификатор сообщения в диалоге.                                 |
/// |`error`                    |`string`   |Сообщение об ошибке, если сообщение не было доставлено получателю. |
/// 
/// Пример ответа:
/// ``` json
/// {
///   "response": 5
/// }
/// ```
/// 
/// # Error's Code
/// 
/// |Code   |Error                                                              |
/// |-------|-------------------------------------------------------------------|
/// |`104`  | Not found                                                         |
/// |`900`  | Can't send messages for users from blacklist                      |
/// |`901`  | Can't send messages for users without permission                  |
/// |`902`  | Can't send messages to this user due to their privacy settings    |
/// |`911`  | Keyboard format is invalid                                        |
/// |`912`  | This is a chat bot feature, change this status in settings        |
/// |`913`  | Too many forwarded messages                                       |
/// |`914`  | Message is too long                                               |
/// |`917`  | You don't have access to this chat                                |
/// |`921`  | Can't forward these messages                                      |
/// |`922`  | You left this chat                                                |
/// |`925`  | You are not admin of this chat                                    |
/// |`936`  | Contact not found                                                 |
/// |`940`  | Too many posts in messages                                        |
/// |`943`  | Cannot use this intent                                            |
/// |`944`  | Limits overflow for this intent                                   |
/// |`945`  | Chat was disabled                                                 |
/// |`946`  | Chat not supported                                                |
/// |`950`  | Can't send message, reply timed out                               |
/// |`962`  | You can't access donut chat without subscription                  |
/// |`969`  | Message cannot be forwarded                                       |
/// |`979`  | App action is restricted for conversations with communities       |
/// 
/// В ходе выполнения могут произойти [общие ошибки](https://dev.vk.com/ru/reference/errors).
pub async fn send(
    api: &Api,
    user_id: usize,
    parameters: SendParameters,
) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/messages.send?access_token={}&v={}&user_id={user_id}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

/// Хранит основные параметры для функции `messages::send(...)`.
#[derive(Debug, Default, Clone)]
pub struct SendParameters {
    /// <b>Обязательный параметр</b>. Идентификатор пользователя, которому отправляется сообщение.
    pub user_id: i64,

    /// <b>Обязательный параметр</b>. Уникальный (в привязке к идентификатору приложения и 
    /// идентификатору отправителя) идентификатор, предназначенный для предотвращения повторной 
    /// отправки одного и того же сообщения. Сохраняется вместе с сообщением и доступен в истории 
    /// сообщений. Возможные значения:
    /// 
    /// * `0` — проверка на уникальность не нужна.
    /// 
    /// * Любое другое число в пределах `int32` — проверка на уникальность нужна.
    /// Переданный в запросе random_id используется 
    /// для проверки уникальности сообщений в заданном диалоге за последний час 
    /// (но не более 100 последних сообщений).
    pub random_id: i64,

    /// <b>Необязательный параметр</b>. Идентификатор получателя сообщения:
    /// 
    /// * Для пользователя — `ИДЕНТИФИКАТОР_ПОЛЬЗОВАТЕЛЯ`.
    /// * Для групповой беседы — `2000000000` + `ИДЕНТИФИКАТОР_БЕСЕДЫ`.
    /// * Для сообщества — `-ИДЕНТИФИКАТОР_СООБЩЕСТВА`.
    pub peer_id: Option<i64>,

    /// <b>Необязательный параметр</b>. Идентификаторы получателей сообщения, 
    /// перечисленные через запятую. Максимальное количество идентификаторов — `100`.
    ///
    /// > Примечание. Доступно только для ключа доступа сообщества.
    pub peer_ids: Option<String>,
    
    /// <b>Необязательный параметр</b>. Короткий адрес пользователя. Пример: `persik_ryzhiy`.
    pub domain: Option<String>,
    
    /// <b>Необязательный параметр</b>. Идентификатор беседы, в которую отправляется сообщение.
    pub chat_id: Option<usize>, 
    
    /// <b>Необязательный параметр</b>. Идентификаторы получателей сообщения, 
    /// перечисленные через запятую. Максимальное количество идентификаторов — `100`.
    ///
    /// > Примечание. Доступно только для ключа доступа сообщества.
    pub user_ids: Option<String>,
    
    /// <b>Необязательный параметр</b>. Текст личного сообщения. 
    /// Максимальное количество символов — `4096`. Обязательный параметр, если не задан параметр `attachment`.
    /// 
    /// Макс. длина = `9000`
    pub message: Option<String>,
    
    
    /// <b>Необязательный параметр</b>. Уникальный идентификатор, 
    /// предназначенный для предотвращения повторной отправки одного и того же сообщения.
    pub guid: Option<i64>,
    
    /// <b>Необязательный параметр</b>. Географическая широта в градусах. Диапазон значений: от `-90` до `90`.
    pub lat: Option<String>,
    
    
    /// <b>Необязательный параметр</b>. Географическая долгота в градусах. Диапазон значений: от `-180` до `180`.
    pub long: Option<String>,
    
    /// <b>Необязательный параметр</b>. Объект или несколько объектов, приложенных к записи. 
    /// Обязательный параметр, если не задан параметр message.
    /// 
    /// К записи можно приложить медиа или ссылку на внешнюю страницу. Если объектов несколько, 
    /// укажите их через запятую `,`.
    /// 
    /// Формат описания медиавложения: `{type}{owner_id}_{media_id}`, где:
    /// 
    /// * `type` — тип медиавложения:
    /// * `photo` — фотография.
    /// * `video` — видеозапись.
    /// * `audio` — аудиозапись.
    /// * `doc` — файл.
    /// * `wall` — запись на стене.
    /// * `market` — товар.
    /// * `poll` — опрос.
    /// * `owner_id` — идентификатор владельца медиавложения. Идентификатор сообщества 
    /// должен начинаться со знака `-`.
    /// * `media_id` — идентификатор медиавложения.
    /// Если прикрепляется медиавложение, которое принадлежит другому пользователю, 
    /// добавьте к формату описания медиавложения ключ доступа: `{type}{owner_id}_{media_id}_{access_key}`.
    /// 
    /// Макс. длина = `9000`
    pub attachment: Option<String>,
    
    /// <b>Необязательный параметр</b>. Идентификатор сообщения, на которое требуется ответить.
    pub reply_to: Option<i64>,
    
    
    /// <b>Необязательный параметр</b>. Идентификаторы пересылаемых сообщений, перечисленные через запятую. 
    /// Пересылаемые сообщения отправителя будут отображаться в теле сообщения у получателя.
    /// 
    /// Ограничения:
    /// 
    /// * Не более `100` значений на верхнем уровне.
    /// * Максимальный уровень вложенности — `45`.
    /// * Максимальное количество пересылаемых сообщений — `500`.
    pub forward_messages: Option<String>,
    
    /// <b>Необязательный параметр</b>. `JSON`-объект со следующими полями:
    /// 
    /// * `owner_id` — владелец сообщений. Укажите это поле, если вы хотите переслать сообщения из сообщества 
    /// в личный диалог.
    /// * `peer_id` — идентификатор места, из которого необходимо переслать сообщения.
    /// * `conversation_message_ids` — массив `conversation_message_id` сообщений, которые необходимо переслать. 
    /// В параметр можно передать сообщения, которые:
    /// Находятся в личном диалоге с ботом.
    /// Являются исходящими сообщениями бота.
    /// Написаны после того, как бот вступил в беседу и появился доступ к сообщениям.
    /// * `message_ids` — массив идентификаторов сообщений.
    /// * `is_reply` — ответ на сообщения. Укажите это поле, если вы хотите ответить на сообщения в том чате, 
    /// в котором находятся сообщения. 
    /// При этом в `conversation_message_ids` или `message_ids` должен находиться только один элемент.
    pub forward: Option<String>,
    
    /// <b>Необязательный параметр</b>. Идентификатор стикера.
    pub sticker_id: Option<usize>,
    
    /// <b>Необязательный параметр</b>. 
    /// Идентификатор сообщества для сообщений сообщества с ключом доступа пользователя.
    pub group_id: Option<i64>,
    
    /// <b>Необязательный параметр</b>. Объект, описывающий 
    /// [клавиатуру бота](https://dev.vk.com/ru/api/bots/development/keyboard).
    pub keyboard: Option<String>,
    
    /// <b>Необязательный параметр</b>. Объект, описывающий 
    /// [шаблон сообщения](https://dev.vk.com/ru/api/bots/development/messages#Шаблоны%20сообщений).
    pub template: Option<String>,
    
    /// <b>Необязательный параметр</b>. Полезные данные.
    pub payload: Option<String>,
    
    /// <b>Необязательный параметр</b>. Объект в формате `JSON`, 
    /// описывающий 
    /// [источник пользовательского контента для чат-ботов](https://dev.vk.com/ru/api/bots/development/messages#Сообщения%20с%20пользовательским%20контентом).
    pub content_source: Option<String>,
    
    /// <b>Необязательный параметр</b>. 
    /// Информация о том, создать ли сниппет ссылки из сообщения. 
    /// 
    /// Возможные значения:
    /// * `1` — не создавать сниппет ссылки из сообщения.
    /// * `0` — создать сниппет ссылки из сообщения.
    pub dont_parse_links: Option<u8>,
    
    /// <b>Необязательный параметр</b>. 
    /// Информация о том, отключить ли уведомление об упоминании в сообщении. 
    ///
    /// Возможные значения:
    /// * `1` — отключить уведомление об упоминании в сообщении.
    /// * `0` — не отключать уведомление об упоминании в сообщении.
    pub disable_mentions: Option<u8>,
    
    /// <b>Необязательный параметр</b>. Строка, описывающая интенты.
    pub intent: Option<String>,
    
    /// <b>Необязательный параметр</b>. Число, которое будет использоваться для работы с интентами в будущем.
    pub subscribe_id: Option<usize>,
}

/// Отправляет событие с действием, которое произойдет при нажатии на callback-кнопку.
pub async fn send_message_event_answer() { unimplemented!() }

/// Метод установки реакции на сообщение
pub async fn send_reaction() { unimplemented!() }

/// Изменяет статус набора текста пользователем в диалоге.
pub async fn set_activity() { unimplemented!() }

/// Метод сохраняет обложку беседы после её успешной 
/// [загрузки на сервер](https://dev.vk.com/ru/api/upload/main-photo-in-chat).
pub async fn set_chat_photo() { unimplemented!() }

/// Старт нового звонка от имени пользователя или от сообщества
pub async fn start_call() { unimplemented!() }

/// Открепляет сообщение.
pub async fn unpin() { unimplemented!() }
