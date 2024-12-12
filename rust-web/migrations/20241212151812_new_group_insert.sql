CREATE OR REPLACE FUNCTION insert_group(group_id int, group_name varchar(100))
RETURNS VOID AS $$
DECLARE
    group_exists BOOLEAN;
BEGIN
    IF (group_name IS NULL) OR (group_name = '') THEN
        RAISE EXCEPTION 'New group name cannot be empty';
    END IF;
    
    SELECT check_group_existence(group_id) INTO group_exists;
    
    IF group_exists THEN
        RAISE EXCEPTION 'The group id exists';
    ELSE
        INSERT INTO groups (id, name) VALUES (group_id, group_name);
    END IF;
END;
$$ LANGUAGE plpgsql;