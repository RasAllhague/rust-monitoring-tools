DROP TABLE IF EXISTS temp_load_averages;

CREATE TABLE IF NOT EXISTS temp_load_averages (
    id_load_average SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    one INT NOT NULL,
    five INT NOT NULL,
    fifteen INT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

INSERT INTO temp_load_averages
SELECT * FROM load_averages;

DELETE FROM load_averages;

ALTER TABLE load_averages
    ALTER COLUMN one TYPE FLOAT,
    ALTER COLUMN five TYPE FLOAT,
    ALTER COLUMN fifteen TYPE FLOAT;

INSERT INTO load_averages
SELECT id_load_average, system_information_id, CAST(one AS FLOAT), CAST(five AS FLOAT), CAST(fifteen AS FLOAT) FROM temp_load_averages;

DROP TABLE temp_load_averages;