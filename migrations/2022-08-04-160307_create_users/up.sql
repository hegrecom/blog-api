CREATE TABLE `users`
(
    `id`              int(8) NOT NULL AUTO_INCREMENT,
    `email`           varchar(255) NOT NULL,
    `password`        varchar(255) NOT NULL,
    `created_at`      datetime DEFAULT CURRENT_TIMESTAMP,
    `updated_at`      datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY               `idx_created_at` (`created_at`),
    KEY               `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `user_tokens`
(
    `id`              int(8) NOT NULL AUTO_INCREMENT,
    `user_id`         int(8) NOT NULL,
    `token`           varchar(255) NOT NULL,
    `expires_at`      datetime NOT NULL,
    `created_at`      datetime DEFAULT CURRENT_TIMESTAMP,
    `updated_at`      datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY               `idx_expires_at` (`expires_at`),
    KEY               `idx_created_at` (`created_at`),
    KEY               `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

