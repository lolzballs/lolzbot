CREATE TABLE paginations (
    message_id BIGINT UNSIGNED PRIMARY KEY NOT NULL,
    command_id BIGINT UNSIGNED NOT NULL,
    current_page INT UNSIGNED NOT NULL DEFAULT 0,

    FOREIGN KEY (command_id) REFERENCES commands(message_id) ON DELETE CASCADE
);
