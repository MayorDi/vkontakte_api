/// Позволяет добавить адрес в сообщество. 
/// Список адресов может быть получен методом [`groups.getAddresses`](https://dev.vk.com/ru/method/groups.getAddresses).
/// > Для того, чтобы воспользоваться этим методом, вы должны быть администратором сообщества.
pub fn add_address() { unimplemented!() }

/// Добавляет сервер для [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started) 
/// в сообщество.
pub fn add_callback_server() { unimplemented!() }

/// Позволяет добавлять ссылки в сообщество.
pub fn add_link() { unimplemented!() }

/// Позволяет одобрить заявку в группу от пользователя.
pub fn approve_request() { unimplemented!() }

/// Добавляет пользователя или группу в черный список сообщества.
pub fn ban() { unimplemented!() }

/// Создает новое сообщество.
pub fn create() { unimplemented!() }

/// Удаляет адрес сообщества.
pub fn delete_address() { unimplemented!() }

/// Удаляет сервер для [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started)  из сообщества.
pub fn delete_callback_server() { unimplemented!() }

/// Позволяет удалить ссылки из сообщества.
pub fn delete_link() { unimplemented!() }

/// Выключает статус «онлайн» в сообществе.
pub fn disable_online() { unimplemented!() }

/// Позволяет редактировать сообщество.
pub fn edit() { unimplemented!() }

/// Метод редактирует адрес в сообществе. 
/// Чтобы получить список адресов, вызовите метод 
/// [`groups.getAddresses`](https://dev.vk.com/ru/method/groups.getAddresses).
pub fn edit_address() { unimplemented!() }

/// Редактирует данные сервера для [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started) в сообществе.
pub fn edit_callback_server() { unimplemented!() }

/// Позволяет редактировать ссылки в сообществе.
pub fn edit_link() { unimplemented!() }

/// Позволяет назначить/разжаловать руководителя в сообществе или изменить уровень его полномочий.
pub fn edit_manager() { unimplemented!() }

/// Позволяет редактировать информацию о месте группы.
pub fn edit_place() { unimplemented!() }

/// Включает статус «онлайн» в сообществе.
pub fn enable_online() { unimplemented!() }

/// Возвращает список сообществ указанного пользователя.
pub fn get() { unimplemented!() }

/// Метод возвращает адрес указанного сообщества.
pub fn get_addresses() { unimplemented!() }

/// Возвращает список забаненных пользователей и сообществ в сообществе.
pub fn get_banned() { unimplemented!() }

/// Возвращает информацию о заданном сообществе или о нескольких сообществах.
pub fn get_by_id() { unimplemented!() }

/// Позволяет получить строку, необходимую для подтверждения адреса сервера в [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started).
pub fn get_callback_confirmation_code() { unimplemented!() }

/// Позволяет получить информацию о настройках сервера для получения уведомлений [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started) в сообществе.
pub fn get_callback_server_settings() { unimplemented!() }

/// Получает информацию о серверах для [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started) в сообществе.
pub fn get_callback_servers() { unimplemented!() }

/// Позволяет получить настройки уведомлений [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started) для сообщества.
pub fn get_callback_settings() { unimplemented!() }

/// Возвращает список сообществ выбранной категории каталога.
pub fn get_catalog() { unimplemented!() }

/// Возвращает список категорий для каталога сообществ.
pub fn get_catalog_info() { unimplemented!() }

/// Возвращает список пользователей, которые были приглашены в группу.
pub fn get_invited_users() { unimplemented!() }

/// Данный метод возвращает список приглашений в сообщества и встречи текущего пользователя.
pub fn get_invites() { unimplemented!() }

/// Возвращает данные для подключения к 
/// [`Bots Longpoll API`](https://dev.vk.com/ru/api/bots-long-poll/getting-started).
pub fn get_long_poll_server() { unimplemented!() }

/// Получает настройки 
/// [`Bots Longpoll API`](https://dev.vk.com/ru/api/bots-long-poll/getting-started)
/// для сообщества.
pub fn get_long_poll_settings() { unimplemented!() }

/// Возвращает список участников сообщества.
pub fn get_members() { unimplemented!() }

/// Получает информацию о статусе «онлайн» в сообществе.
pub fn get_online_status() { unimplemented!() }

/// Возвращает список заявок на вступление в сообщество.
pub fn get_requests() { unimplemented!() }

/// Позволяет получать данные, необходимые для отображения страницы редактирования данных сообщества.
pub fn get_settings() { unimplemented!() }

/// Возвращает список тегов сообщества
pub fn get_tag_list() { unimplemented!() }

/// Возвращает настройки прав для ключа доступа сообщества.
pub fn get_token_permissions() { unimplemented!() }

/// Позволяет приглашать друзей в группу.
pub fn invite() { unimplemented!() }

/// Возвращает информацию о том, является ли пользователь участником сообщества.
pub fn is_member() { unimplemented!() }

/// Данный метод позволяет вступить в группу, 
/// публичную страницу, а также подтвердить участие во встрече.
pub fn join() { unimplemented!() }

/// Позволяет покинуть сообщество или отклонить приглашение в сообщество.
pub fn leave() { unimplemented!() }

/// Позволяет исключить пользователя из группы или отклонить заявку на вступление.
pub fn remove_user() { unimplemented!() }

/// Позволяет менять местоположение ссылки в списке.
pub fn reorder_link() { unimplemented!() }

/// Осуществляет поиск сообществ по заданной подстроке.
pub fn search() { unimplemented!() }

/// Позволяет задать настройки уведомлений о событиях в [`Callback API`](https://dev.vk.com/ru/api/callback/getting-started).
pub fn set_callback_settings() { unimplemented!() }

/// Задаёт настройки для Bots Long Poll API в сообществе.
pub fn set_long_poll_settings() { unimplemented!() }

/// Устанавливает настройки сообщества
pub fn set_settings() { unimplemented!() }

/// Позволяет создать или отредактировать заметку о пользователе в 
/// рамках переписки пользователя с сообществом
pub fn set_user_note() { unimplemented!() }

/// Позволяет добавить новый тег в сообщество.
pub fn tag_add() { unimplemented!() }

/// Позволяет «привязывать» и «отвязывать» теги сообщества к беседам.
pub fn tag_bind() { unimplemented!() }

/// Позволяет удалить тег сообщества.
pub fn tag_delete() { unimplemented!() }

/// Позволяет переименовать существующий тег.
pub fn tag_update() { unimplemented!() }

/// Переключает функционал раздела «Товаров» в выбранной группе.
pub fn toggle_market() { unimplemented!() }

/// Убирает пользователя или группу из черного списка сообщества.
pub fn unban() { unimplemented!() }