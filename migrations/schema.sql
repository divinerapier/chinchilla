CREATE TABLE IF NOT EXISTS `post`
(
    `id`        INT AUTO_INCREMENT,
    `uuid`      CHAR(36)     NOT NULL,
    `title`     VARCHAR(127) NOT NULL, -- display at frontend
    `link_name` VARCHAR(127) NOT NULL, -- url link
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `post_tag`
(
    `id`        INT AUTO_INCREMENT,
    `post_uuid` CHAR(36)    NOT NULL,
    `tag`       VARCHAR(63) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`)
);
