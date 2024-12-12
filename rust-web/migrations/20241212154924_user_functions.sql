-- Check user item exists
CREATE OR REPLACE FUNCTION check_user_existence(user_id INT)
RETURNS TEXT AS $$
DECLARE
    user_exists BOOLEAN;
BEGIN
    SELECT EXISTS(SELECT 1 FROM users WHERE id = user_id) INTO user_exists;
    RETURN user_exists;
END;
$$ LANGUAGE plpgsql;

-- Insert group
CREATE OR REPLACE FUNCTION insert_user(user_name varchar(100), user_email varchar(100), user_password varchar(100))
RETURNS VOID AS $$
BEGIN
    IF (user_name IS NULL) OR (user_name = '') THEN
        RAISE EXCEPTION 'New user name cannot be empty';
    END IF;
    IF (user_email IS NULL) OR (user_email = '') THEN
        RAISE EXCEPTION 'New user email cannot be empty';
    END IF;
    IF (user_password IS NULL) OR (user_password = '') THEN
        RAISE EXCEPTION 'New user password cannot be empty';
    END IF;
    INSERT INTO users (name, email, password) VALUES (user_name, user_email, user_password);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION insert_user(user_id int, user_name varchar(100), user_email varchar(100), user_password varchar(100))
RETURNS VOID AS $$
DECLARE
    user_exists BOOLEAN;
BEGIN
    IF (user_name IS NULL) OR (user_name = '') THEN
        RAISE EXCEPTION 'User name cannot be empty';
    END IF;

    IF (user_email IS NULL) OR (user_email = '') THEN
        RAISE EXCEPTION 'User email cannot be empty';
    END IF;

    IF (user_password IS NULL) OR (user_password = '') THEN
        RAISE EXCEPTION 'User password cannot be empty';
    END IF;
    
    SELECT check_user_existence(user_id) INTO user_exists;
    IF user_exists THEN
        RAISE EXCEPTION 'The user id exists';
    ELSE
        INSERT INTO users (id, name, email, password) VALUES (user_id, user_name, user_email, user_password);
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Update group
CREATE FUNCTION update_user(user_id INT, new_user_name varchar(100), new_user_email varchar(100), new_user_password varchar(100))
RETURNS VOID AS $$
DECLARE
    user_exists BOOLEAN;
BEGIN
    IF (new_user_name IS NULL) OR (new_user_name = '') THEN
        RAISE EXCEPTION 'User name cannot be empty';
    END IF;

    IF (new_user_email IS NULL) OR (new_user_email = '') THEN
        RAISE EXCEPTION 'User email cannot be empty';
    END IF;

    IF (new_user_password IS NULL) OR (new_user_password = '') THEN
        RAISE EXCEPTION 'User password cannot be empty';
    END IF;
    
    SELECT check_user_existence(user_id) INTO user_exists;
    IF user_exists THEN
        UPDATE users SET name = new_user_name, email = new_user_email, password = new_user_password WHERE id = user_id;
    ELSE
        RAISE EXCEPTION 'The user does not exists';
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Delete group
CREATE OR REPLACE FUNCTION delete_user(user_id INT)
RETURNS VOID AS $$
DECLARE
    user_exists BOOLEAN;
BEGIN
    SELECT check_user_existence(user_id) INTO user_exists;
    IF user_exists THEN
        DELETE FROM users WHERE id = user_id;
    ELSE
        RAISE EXCEPTION 'User does not exist';
    END IF;
END;
$$ LANGUAGE plpgsql;