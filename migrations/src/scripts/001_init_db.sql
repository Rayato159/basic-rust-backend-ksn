-- Add migration script here
-- Create Users Table
CREATE TABLE users (
    id UNIQUEIDENTIFIER PRIMARY KEY,
    username NVARCHAR(50) NOT NULL UNIQUE,
    email NVARCHAR(100) NOT NULL UNIQUE,
    password_hash NVARCHAR(255) NOT NULL,
    created_at DATETIMEOFFSET NOT NULL,
    updated_at DATETIMEOFFSET NULL
);

-- Create Transactions Table
CREATE TABLE transactions (
    id UNIQUEIDENTIFIER PRIMARY KEY,
    user_id UNIQUEIDENTIFIER NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    currency NVARCHAR(3) NOT NULL DEFAULT 'THB',
    status NVARCHAR(20) NOT NULL,
    created_at DATETIMEOFFSET NOT NULL,
    updated_at DATETIMEOFFSET NULL,

    -- สร้าง Relationship
    CONSTRAINT FK_transactions_users FOREIGN KEY (user_id)
    REFERENCES users(id) ON DELETE CASCADE
);
