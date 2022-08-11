CREATE TABLE `posts`
(
    `id`              int(8) NOT NULL AUTO_INCREMENT,
    `title`           varchar(255) NOT NULL,
    `body`            text NOT NULL,
    `status`          enum('draft', 'published') NOT NULL DEFAULT 'draft',
    `created_at`      datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`      datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY               `idx_created_at` (`created_at`),
    KEY               `idx_updated_at` (`updated_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

