-- Group table
CREATE TABLE IF NOT EXISTS groups (
    id serial PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

-- Check group item exists
CREATE OR REPLACE FUNCTION check_group_existence(group_id INT)
RETURNS TEXT AS $$
DECLARE
    group_exists BOOLEAN;
BEGIN
    SELECT EXISTS(SELECT 1 FROM groups WHERE id = group_id) INTO group_exists;
    RETURN group_exists;
END;
$$ LANGUAGE plpgsql;

-- Insert group
CREATE OR REPLACE FUNCTION insert_group(group_name varchar(100))
RETURNS VOID AS $$
BEGIN
    IF (group_name IS NULL) OR (group_name = '') THEN
        RAISE EXCEPTION 'New group name cannot be empty';
    END IF;
    INSERT INTO groups (name) VALUES (group_name);
END;
$$ LANGUAGE plpgsql;

-- Update group
CREATE FUNCTION update_group(group_id INT, new_name VARCHAR(100))
RETURNS VOID AS $$
DECLARE
    group_exists BOOLEAN;
BEGIN
    IF (new_name IS NULL) OR (new_name = '') THEN
        RAISE EXCEPTION 'New group name cannot be empty';
    END IF;
    SELECT check_group_existence(group_id) INTO group_exists;
    IF group_exists THEN
        UPDATE groups SET name = new_name WHERE id = group_id;
    ELSE
        RAISE EXCEPTION 'Group does not exist';
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Delete group
CREATE OR REPLACE FUNCTION delete_group(group_id INT)
RETURNS VOID AS $$
DECLARE
    group_exists BOOLEAN;
BEGIN
    SELECT check_group_existence(group_id) INTO group_exists;
    IF group_exists THEN
        DELETE FROM groups WHERE id = group_id;
    ELSE
        RAISE EXCEPTION 'Group does not exist';
    END IF;
END;
$$ LANGUAGE plpgsql;