delete from filament;
delete from product;
delete from material;
delete from vendor;

alter sequence filament_id_filament_seq restart with 1;
alter sequence material_id_material_seq restart with 1;
alter sequence product_id_product_seq restart with 1;
alter sequence vendor_id_vendor_seq restart with 1;

-------------
-- VENDORS --
-------------

insert into vendor (name_vendor)
values ('Bambu Lab');
insert into vendor (name_vendor)
values ('Filament PM');
insert into vendor (name_vendor)
values ('eSun');
insert into vendor (name_vendor)
values ('Polymaker');
insert into vendor (name_vendor)
values ('Prusa');
insert into vendor (name_vendor)
values ('Fiberlogy');
insert into vendor (name_vendor)
values ('Spectrum');
insert into vendor (name_vendor)
values ('Alza');

---------------
-- MATERIALS --
---------------

insert into material (name_material)
values ('PLA');
insert into material (name_material)
values ('PETG');
insert into material (name_material)
values ('TPU');

--------------
-- PRODUCTS --
--------------

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Bambu Lab'),
        (select id_material from material where name_material = 'PLA'),
        'PLA Basic',
        190, 250,
        45, 65);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Filament PM'),
        (select id_material from material where name_material = 'PLA'),
        'PLA 1.75',
        200, 220,
        25, 60);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Filament PM'),
        (select id_material from material where name_material = 'PLA'),
        'PLA+ 1.75',
        190, 210,
        60, NULL);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Filament PM'),
        (select id_material from material where name_material = 'PLA'),
        'PLA+ Army Edition',
        190, 250,
        60, NULL);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'eSun'),
        (select id_material from material where name_material = 'PLA'),
        'PLA+',
        205, 225,
        60, 80);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'eSun'),
        (select id_material from material where name_material = 'PLA'),
        'eSilk-PLA',
        190, 220,
        60, 80);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'eSun'),
        (select id_material from material where name_material = 'PLA'),
        'ePLA-Silk Magic',
        190, 230,
        45, 60);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Polymaker'),
        (select id_material from material where name_material = 'PLA'),
        'PolyTerra PLA',
        190, 230,
        25, 60);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Prusa'),
        (select id_material from material where name_material = 'PLA'),
        'Prusament PLA',
        205, 225,
        40, 60);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Fiberlogy'),
        (select id_material from material where name_material = 'TPU'),
        'FiberFlex 40D',
        200, 220,
        50, 70);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Spectrum'),
        (select id_material from material where name_material = 'PETG'),
        'PET-G Premium',
        230, 255,
        60, 80);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Spectrum'),
        (select id_material from material where name_material = 'PETG'),
        'PET-G Matt',
        230, 255,
        60, 80);

insert into product (id_vendor, id_material, name_product, temp_min, temp_max, temp_bed_min, temp_bed_max)
values ((select id_vendor from vendor where name_vendor = 'Alza'),
        (select id_material from material where name_material = 'PLA'),
        'Alzament PLA Basic',
        220, NULL,
        45, 60);

---------------
-- FILAMENTS --
---------------

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        29.99, 'Black', '#111111',
        0, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        29.99, 'Red', '#c12e1f',
        287, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        19.99, 'Purple', '#5e43b7',
        963, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        19.99, 'Orange', '#ff6a13',
        955, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        19.99, 'Pink', '#f55a74',
        985, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        19.99, 'Brown', '#9c432c',
        618, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Bambu Lab')
           and name_product = 'PLA Basic'),
        19.99, 'Beige', '#f7e6de',
        981, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Filament PM')
           and name_product = 'PLA+ 1.75'),
        23.90, 'White', '#eeeeee',
        0, 216);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Filament PM')
           and name_product = 'PLA+ 1.75'),
        23.90, 'Sweet Mint', '#73bab5',
        0, 216);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Filament PM')
           and name_product = 'PLA 1.75'),
        26.90, 'Green', '#80bf1a',
        198, 216);

insert into filament (id_product, price, color_name, color_hex, netto_weight, original_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Filament PM')
           and name_product = 'PLA+ Army Edition'),
        12.99, 'Dusty Brown', '#a69281',
        170, 500, 230);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'eSun')
           and name_product = 'PLA+'),
        20.99, 'Yellow', '#fbe625',
        0, 224);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'eSun')
           and name_product = 'eSilk-PLA'),
        20.99, 'Blue', '#123cea',
        409, 224);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'eSun')
           and name_product = 'ePLA-Silk Magic'),
        25.99, 'Blue', '#e41e95',
        658, 220);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Polymaker')
           and name_product = 'PolyTerra PLA'),
        19.99, 'Sakura Pink', '#e4bdd0',
        191, 140);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Polymaker')
           and name_product = 'PolyTerra PLA'),
        19.99, 'Fossil Gray', '#aaaaaa',
        313, 140);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Polymaker')
           and name_product = 'PolyTerra PLA'),
        19.99, 'Charcoal Black', '#111111',
        55, 140);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Polymaker')
           and name_product = 'PolyTerra PLA'),
        19.99, 'Cotton White', '#eeeeee',
        0, 140);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Polymaker')
           and name_product = 'PolyTerra PLA'),
        19.99, 'Sapphire Blue', '#026bbf',
        759, 140);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Prusa')
           and name_product = 'Prusament PLA'),
        29.99, 'Prusa Orange', '#fc6d09',
        296, 186);

insert into filament (id_product, price, color_name, color_hex, netto_weight, original_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Fiberlogy')
           and name_product = 'FiberFlex 40D'),
        25.30, 'Black', '#111111',
        227, 500, 250);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Spectrum')
           and name_product = 'PET-G Premium'),
        19.99, 'Deep Black', '#141414',
        551, 260);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Spectrum')
           and name_product = 'PET-G Matt'),
        19.99, 'Polar White', '#eeeeee',
        520, 260);

insert into filament (id_product, price, color_name, color_hex, netto_weight, spool_weight)
values ((select id_product
         from product
         where id_vendor = (select id_vendor from vendor where name_vendor = 'Alza')
           and name_product = 'Alzament PLA Basic'),
        9.99, 'Yellow', '#fefe01',
        853, 137);
