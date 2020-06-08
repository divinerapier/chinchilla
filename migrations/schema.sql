CREATE TABLE IF NOT EXISTS `post`
(
    `id`         INT AUTO_INCREMENT,
    `uuid`       CHAR(36)     NOT NULL,
    `title`      VARCHAR(127) NOT NULL, -- display at frontend
    `link_name`  VARCHAR(127) NOT NULL, -- url link
    `created_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8;

ALTER TABLE `post`
    ADD UNIQUE INDEX unique_link_name (`link_name`);

CREATE TABLE IF NOT EXISTS `post_tag`
(
    `id`         INT AUTO_INCREMENT,
    `post_uuid`  CHAR(36)    NOT NULL,
    `tag`        VARCHAR(63) NOT NULL DEFAULT '',
    `created_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8;

ALTER TABLE `post_tag`
    ADD INDEX index_tag (`tag`);
