CREATE SCHEMA IF NOT EXISTS `vitrine-digital` ;
USE `vitrine-digital` ;

-- -----------------------------------------------------
-- Table `vitrine-digital`.`client`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `vitrine-digital`.`client` (
  `client_id` INT NOT NULL AUTO_INCREMENT,
  `client_name` VARCHAR(200) NOT NULL,
  `client_mail` VARCHAR(65) NULL,
  `client_phone` VARCHAR(45) NULL,
  `client_uf` VARCHAR(2) NULL,
  `client_city` VARCHAR(60) NULL,
  `client_address` VARCHAR(125) NULL,
  PRIMARY KEY (`client_id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `vitrine-digital`.`client_demand`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `vitrine-digital`.`client_demand` (
  `client_demand_id` INT NOT NULL AUTO_INCREMENT,
  `client_demand_date` DATETIME NOT NULL,
  `client_client_id` INT NOT NULL,
  PRIMARY KEY (`client_demand_id`),
  INDEX `fk_client_demand_client1_idx` (`client_client_id` ASC) VISIBLE,
  CONSTRAINT `fk_client_demand_client1`
    FOREIGN KEY (`client_client_id`)
    REFERENCES `vitrine-digital`.`client` (`client_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `vitrine-digital`.`product`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `vitrine-digital`.`product` (
  `product_id` INT NOT NULL AUTO_INCREMENT,
  `product_description` VARCHAR(255) NOT NULL,
  `product_value` FLOAT NULL,
  `product_second_value` FLOAT NULL,
  `product_image` TEXT NULL,
  `client_demand_client_demand_id` INT NOT NULL,
  PRIMARY KEY (`product_id`),
  INDEX `fk_product_client_demand1_idx` (`client_demand_client_demand_id` ASC) VISIBLE,
  CONSTRAINT `fk_product_client_demand1`
    FOREIGN KEY (`client_demand_client_demand_id`)
    REFERENCES `vitrine-digital`.`client_demand` (`client_demand_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `vitrine-digital`.`art`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `vitrine-digital`.`art` (
  `art_id` INT NOT NULL AUTO_INCREMENT,
  `art_delivery_time` DATETIME NOT NULL,
  `art_image` TEXT NOT NULL,
  `client_demand_client_demand_id` INT NOT NULL,
  PRIMARY KEY (`art_id`),
  INDEX `fk_art_client_demand1_idx` (`client_demand_client_demand_id` ASC) VISIBLE,
  CONSTRAINT `fk_art_client_demand1`
    FOREIGN KEY (`client_demand_client_demand_id`)
    REFERENCES `vitrine-digital`.`client_demand` (`client_demand_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `vitrine-digital`.`login`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `vitrine-digital`.`login` (
  `login_id` INT NOT NULL AUTO_INCREMENT,
  `login_mail` VARCHAR(65) NOT NULL,
  `login_password` VARCHAR(45) NOT NULL,
  `logincol` VARCHAR(255) NULL,
  `client_client_id` INT NOT NULL,
  PRIMARY KEY (`login_id`),
  INDEX `fk_login_client1_idx` (`client_client_id` ASC) VISIBLE,
  CONSTRAINT `fk_login_client1`
    FOREIGN KEY (`client_client_id`)
    REFERENCES `vitrine-digital`.`client` (`client_id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;