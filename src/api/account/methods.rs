/// Добавляет пользователя или группу в черный список.
pub fn ban() { unimplemented!() }

/// Позволяет сменить пароль пользователя после успешного восстановления доступа к аккаунту через СМС,
/// используя метод [`auth.restore`](https://dev.vk.com/ru/method/auth.restore).
pub fn change_password() { unimplemented!() }

/// Возвращает список активных рекламных предложений (офферов), 
/// выполнив которые, пользователь сможет получить соответствующее количество голосов 
/// на свой счёт внутри приложения.
pub fn get_active_offers() { unimplemented!() }

/// Метод получает настройки пользователя вашего [приложения](https://vk.com/apps?act=manage).
pub fn get_app_permissions() { unimplemented!() }

/// Возвращает список пользователей, находящихся в черном списке.
pub fn get_banned() { unimplemented!() }

/// Метод возвращает ненулевые значения счётчиков пользователя.
pub fn get_counters() { unimplemented!() }

/// Возвращает информацию о текущем аккаунте.
pub fn get_info() { unimplemented!() }

/// Возвращает информацию о текущем профиле.
pub fn get_profile_info() { unimplemented!() }

/// Позволяет получать настройки Push-уведомлений.
pub fn get_push_settings() { unimplemented!() }

/// Позволяет искать пользователей ВКонтакте, 
/// используя телефонные номера, email-адреса, 
/// и идентификаторы пользователей в других сервисах. 
/// Найденные пользователи могут быть также в 
/// дальнейшем получены методом [`friends.getSuggestions`](https://dev.vk.com/ru/method/friends.getSuggestions).
pub fn lookup_contacts() { unimplemented!() }

/// Подписывает устройство на базе iOS, Android, Windows Phone или Mac на получение push-уведомлений.
pub fn register_device() { unimplemented!() }

/// Редактирует информацию текущего профиля.
pub fn save_profile_info() { unimplemented!() }

/// Позволяет редактировать информацию о текущем аккаунте.
pub fn set_info() { unimplemented!() }

/// Устанавливает короткое название приложения (до 17 символов), 
/// которое выводится пользователю в левом меню.
pub fn set_name_in_menu() { unimplemented!() }

/// Помечает текущего пользователя как `offline` 
/// (только в текущем приложении).
pub fn set_offline() { unimplemented!() }

/// Помечает текущего пользователя как `online` на 5 минут.
pub fn set_online() { unimplemented!() }

/// Изменяет настройку push-уведомлений.
pub fn set_push_settings() { unimplemented!() }

/// Отключает push-уведомления на заданный промежуток времени.
pub fn set_silence_mode() { unimplemented!() }

/// Удаляет пользователя или группу из черного списка.
pub fn unban() { unimplemented!() }

/// Отписывает устройство от Push уведомлений.
pub fn unregister_device() { unimplemented!() }