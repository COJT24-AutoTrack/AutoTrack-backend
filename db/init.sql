CREATE DATABASE IF NOT EXISTS `auto_track-db`;
CREATE USER IF NOT EXISTS 'user'@'%' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON `auto_track-db`.* TO 'user'@'%';
FLUSH PRIVILEGES;

USE `auto_track-db`;

CREATE TABLE Users (
    user_id INT AUTO_INCREMENT PRIMARY KEY,
    user_email VARCHAR(255) NOT NULL UNIQUE,
    user_name VARCHAR(255) NOT NULL,
    user_password VARCHAR(255) NOT NULL,
    firebase_user_id VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE Cars (
    car_id INT AUTO_INCREMENT PRIMARY KEY,
    car_name VARCHAR(255) NOT NULL,
    carmodelnum VARCHAR(255) NOT NULL,
    car_color VARCHAR(255) NOT NULL,
    car_mileage INT NOT NULL,
    car_isflooding BOOLEAN NOT NULL,
    car_issmoked BOOLEAN NOT NULL,
    car_image_url VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE user_car (
    user_id INT NOT NULL,
    car_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    PRIMARY KEY (user_id, car_id),
    INDEX (user_id),
    INDEX (car_id)
);

CREATE TABLE Tunings (
    tuning_id INT AUTO_INCREMENT PRIMARY KEY,
    car_id INT NOT NULL,
    tuning_name VARCHAR(255) NOT NULL,
    tuning_date DATE NOT NULL,
    tuning_description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    INDEX (car_id)
);

CREATE TABLE Maintenances (
    maint_id INT AUTO_INCREMENT PRIMARY KEY,
    car_id INT NOT NULL,
    maint_type VARCHAR(255) NOT NULL,
    maint_date DATE NOT NULL,
    maint_description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    INDEX (car_id)
);

CREATE TABLE FuelEfficiencies (
    fe_id INT AUTO_INCREMENT PRIMARY KEY,
    car_id INT NOT NULL,
    fe_date DATE NOT NULL,
    fe_amount FLOAT NOT NULL,
    fe_unitprice FLOAT NOT NULL,
    fe_mileage INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    INDEX (car_id)
);

CREATE TABLE Accidents (
    accident_id INT AUTO_INCREMENT PRIMARY KEY,
    car_id INT NOT NULL,
    accident_date DATE NOT NULL,
    accident_description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    INDEX (car_id)
);

CREATE TABLE PeriodicInspection (
    pi_id INT AUTO_INCREMENT PRIMARY KEY,
    car_id INT NOT NULL,
    pi_name VARCHAR(255) NOT NULL,
    pi_date DATE NOT NULL,
    pi_nextdate DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (car_id) REFERENCES Cars(car_id),
    INDEX (car_id)
);
