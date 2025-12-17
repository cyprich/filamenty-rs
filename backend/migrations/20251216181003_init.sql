-- Add migration script here

-- drop table if exists filament;
-- drop table if exists product;
-- drop table if exists material;
-- drop table if exists vendor;
-- drop domain if exists temperature_celsius;
-- drop domain if exists grams;

create domain grams as integer
check (VALUE >= 0);

create domain temperature_celsius as integer
check (VALUE >= 0);

create table vendor (
    id_vendor serial primary key,
    name_vendor varchar(50) not null,
    unique (name_vendor)
);

create table material (
    id_material serial primary key,
    name_material varchar(50) not null,
    unique (name_material)
);

create table product (
    id_product serial primary key,
    id_vendor int not null,
    id_material int not null,
    name_product varchar(50) not null,
    diameter float default 1.75 not null,
    temp_min temperature_celsius not null,
    temp_max temperature_celsius,
    temp_bed_min temperature_celsius not null,
    temp_bed_max temperature_celsius,
    -- TODO: speed?
    foreign key (id_vendor) references vendor(id_vendor),
    foreign key (id_material) references material(id_material)
);

create table filament (
    id_filament serial primary key,
    id_product int not null,
    price float not null,
    color_name varchar(50) not null,
    color_hex char(7) not null,
    original_weight grams default 1000,
    netto_weight grams not null,
    spool_weight grams not null,
    last_update timestamp default now() not null,
    foreign key (id_product) references product (id_product)
);
