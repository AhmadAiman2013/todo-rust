-- Add migration script here
CREATE TABLE tasks (
                       id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
                       user_id VARCHAR(100) NOT NULL,
                       title VARCHAR(255) NOT NULL,
                       description TEXT,
                       due_date DATETIME NULL,
                       status BOOLEAN NOT NULL DEFAULT FALSE,
                       recurrence_rule ENUM('NONE', 'DAILY', 'WEEKLY', 'MONTHLY') NOT NULL DEFAULT 'NONE',
                       recurrence_start_date DATETIME NULL,
                       recurrence_end_date DATETIME NULL,
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
