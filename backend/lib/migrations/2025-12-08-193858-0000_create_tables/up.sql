-- Your SQL goes here

create domain grams as numeric(4)
check (
    VALUE > 0
);



create table vendor (
    id_vendor serial primary key,
    name_vendor varchar(50) not null
);

create table material (
    id_material serial primary key,
    name_material varchar(50) not null
);

create table spool (
    id_spool serial primary key,
    id_vendor int not null,
    empty_weight grams not null,
    foreign key (id_vendor) references vendor(id_vendor)
);

create table filament (
    id_filament serial primary key,
    id_vendor int not null,
    id_material int not null,
    id_spool int not null,
    price numeric(5,2) not null,
    color_name varchar(50) not null,
    color_hex char(7) not null,
    temp_min numeric(3) not null,
    temp_max numeric(3),
    temp_bed_min numeric(3) not null,
    temp_bed_max numeric(3),
    original_weight grams,
    current_weight grams,
    foreign key (id_vendor) references vendor(id_vendor),
    foreign key (id_material) references material(id_material),
    foreign key (id_spool) references spool(id_spool)
);

