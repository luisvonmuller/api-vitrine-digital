-- Your SQL goes here

ALTER TABLE `art`
MODIFY COLUMN `art_delivery_time` timestamp NOT NULL;


ALTER TABLE `client_demand`
MODIFY COLUMN `client_demand_date` timestamp NOT NULL;