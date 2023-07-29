//! # Messages
//! Методы для работы с личными сообщениями. <br>
//! 
//! Для моментального получения входящих сообщений используйте 
//! [LongPoll сервер](https://dev.vk.com/ru/api/user-long-poll/getting-started).<br>
//! 
//! > <b>Обратите внимание</b>: доступ к работе с методами секции с ключом пользователя ограничен. <br>
//! 
//! Информация об ограничении Messages API находится в 
//! [Roadmap](https://dev.vk.com/ru/reference/roadmap#Ограничение%20Messages%20API). <br>
//! 
//! Обратите внимание: методы для работы со звонками были перенесены в новую секцию 
//! [calls](https://dev.vk.com/ru/method/calls). 
//! Старые методы звонков из секции messages были помечены устаревшими и могут быть удалены в будущих версиях API.

mod methods;
pub use methods::*;