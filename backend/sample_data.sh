#!/usr/bin/env bash

BASE_URL="http://localhost:5000/api/v2"

echo "WARNING! This script expects the database to be completely empty (including generated ID's), and the base URL to be ${BASE_URL}"
read -p "Do you want to continue? (y/N)" -n 1 answer

if [[ $answer == "y" || $answer == "Y" ]]; then
  echo
  echo "Running..."
else
  echo
  echo "Exiting..."
  exit 1
fi

# curl -v -X POST \
#       http://localhost:5000/api/v2/filaments-with-image \
#       -F 'json={"id_product": 2, "price": 19.99, "color_name": "black", "color_hex": "#111111", "original_weight": 1000, "netto_weight": 1000, "spool_weight": 1000};type=application/json' \
#       -F image=@/home/cyprich/Coding/Rust/images.bak/filament2.png

echo ""
echo ""
echo "Materials..."
curl -X POST "$BASE_URL/materials" -d '{"name_material": "PLA"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/materials" -d '{"name_material": "PETG"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/materials" -d '{"name_material": "TPU"}' -H "Content-Type: application/json"

echo ""
echo ""
echo "Vendors..."
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Bambu Lab"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Filament PM"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "eSun"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "PolyMaker"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Prusa"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Fiberlogy"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Spectrum Filaments"}' -H "Content-Type: application/json"
curl -X POST "$BASE_URL/vendors" -d '{"name_vendor": "Alza"}' -H "Content-Type: application/json"

echo ""
echo ""
echo "Products..."
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 1, "id_material": 1, "name_product": "PLA Basic", "temp_min": 190, "temp_max": 230, "temp_bed_min": 45, "temp_bed_max": 65, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 2, "id_material": 1, "name_product": "PLA 1.75", "temp_min": 200, "temp_max": 220, "temp_bed_min": 25, "temp_bed_max": 60, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 2, "id_material": 1, "name_product": "PLA+ 1.75", "temp_min": 190, "temp_max": 210, "temp_bed_min": 25, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 2, "id_material": 1, "name_product": "PLA+ Army Edition", "temp_min": 190, "temp_max": 230, "temp_bed_min": 25, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 3, "id_material": 1, "name_product": "PLA+", "temp_min": 205, "temp_max": 225, "temp_bed_min": 60, "temp_bed_max": 80, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 3, "id_material": 1, "name_product": "eSilk-PLA", "temp_min": 190, "temp_max": 220, "temp_bed_min": 60, "temp_bed_max": 80, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 3, "id_material": 1, "name_product": "ePLA-Silk Magic", "temp_min": 190, "temp_max": 230, "temp_bed_min": 45, "temp_bed_max": 60, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 4, "id_material": 1, "name_product": "PolyTerra PLA", "temp_min": 190, "temp_max": 230, "temp_bed_min": 25, "temp_bed_max": 60, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 5, "id_material": 1, "name_product": "Prusament PLA", "temp_min": 205, "temp_max": 225, "temp_bed_min": 40, "temp_bed_max": 60, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 6, "id_material": 3, "name_product": "FiberFlex 40D", "temp_min": 200, "temp_max": 220, "temp_bed_min": 50, "temp_bed_max": 70, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 7, "id_material": 2, "name_product": "PETG Premium", "temp_min": 230, "temp_max": 255, "temp_bed_min": 60, "temp_bed_max": 80, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 7, "id_material": 2, "name_product": "PETG Matt", "temp_min": 230, "temp_max": 255, "temp_bed_min": 60, "temp_bed_max": 80, "diameter": 1.75}'
curl -X POST "$BASE_URL/products" -H "Content-Type: application/json" -d '{"id_vendor": 8, "id_material": 2, "name_product": "Alzament PLA Basic", "temp_min": 220, "temp_bed_min": 45, "temp_bed_max": 60, "diameter": 1.75}'

echo ""
echo ""
echo "Filaments..."
curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 29.99, "color_name": "Black", "color_hex": "#111111", "original_weight": 1000, "netto_weight": 0, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament1.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 29.99, "color_name": "Red", "color_hex": "#c12e1f", "original_weight": 1000, "netto_weight": 287, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament2.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 3, "price": 23.90, "color_name": "White", "color_hex": "#eeeeee", "original_weight": 1000, "netto_weight": 0, "spool_weight": 216};type=application/json' \
  -F image=@./sample_images/filament3.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 4, "price": 23.90, "color_name": "Sweet Mint", "color_hex": "#73bab5", "original_weight": 1000, "netto_weight": 575, "spool_weight": 216};type=application/json' \
  -F image=@./sample_images/filament4.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 5, "price": 20.99, "color_name": "Yellow", "color_hex": "#fbe625", "original_weight": 1000, "netto_weight": 0, "spool_weight": 224};type=application/json' \
  -F image=@./sample_images/filament5.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 6, "price": 20.99, "color_name": "Blue", "color_hex": "#123cea", "original_weight": 1000, "netto_weight": 409, "spool_weight": 224};type=application/json' \
  -F image=@./sample_images/filament6.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 7, "price": 25.99, "color_name": "Blue-Pink", "color_hex": "#e41e95", "original_weight": 1000, "netto_weight": 658, "spool_weight": 220};type=application/json' \
  -F image=@./sample_images/filament7.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 8, "price": 19.99, "color_name": "Sakura Pink", "color_hex": "#e4bdd0", "original_weight": 1000, "netto_weight": 191, "spool_weight": 140};type=application/json' \
  -F image=@./sample_images/filament8.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 8, "price": 19.99, "color_name": "Fossil Gray", "color_hex": "#aaaaaa", "original_weight": 1000, "netto_weight": 313, "spool_weight": 140};type=application/json' \
  -F image=@./sample_images/filament9.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 2, "price": 26.90, "color_name": "Green", "color_hex": "#80bf1a", "original_weight": 1000, "netto_weight": 198, "spool_weight": 216};type=application/json' \
  -F image=@./sample_images/filament10.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 9, "price": 29.99, "color_name": "Prusa Orange", "color_hex": "#fc6d09", "original_weight": 1000, "netto_weight": 296, "spool_weight": 186};type=application/json' \
  -F image=@./sample_images/filament11.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 8, "price": 19.99, "color_name": "Charcoal Black", "color_hex": "#111111", "original_weight": 1000, "netto_weight": 55, "spool_weight": 140};type=application/json' \
  -F image=@./sample_images/filament12.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 8, "price": 19.99, "color_name": "Cotton White", "color_hex": "#eeeeee", "original_weight": 1000, "netto_weight": 0, "spool_weight": 140};type=application/json' \
  -F image=@./sample_images/filament13.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 10, "price": 25.30, "color_name": "Black", "color_hex": "#111111", "original_weight": 500, "netto_weight": 227, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament14.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 4, "price": 26.90, "color_name": "Dusty Brown", "color_hex": "#a69281", "original_weight": 500, "netto_weight": 170, "spool_weight": 230};type=application/json' \
  -F image=@./sample_images/filament15.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 8, "price": 19.99, "color_name": "Sapphire Blue", "color_hex": "#026bbf", "original_weight": 1000, "netto_weight": 759, "spool_weight": 140};type=application/json' \
  -F image=@./sample_images/filament16.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 19.99, "color_name": "Purple", "color_hex": "#5e43b7", "original_weight": 1000, "netto_weight": 963, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament17.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 11, "price": 19.99, "color_name": "Deep Black", "color_hex": "#141414", "original_weight": 1000, "netto_weight": 551, "spool_weight": 260};type=application/json' \
  -F image=@./sample_images/filament18.jpg

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 12, "price": 19.99, "color_name": "Polar White", "color_hex": "#eeeeee", "original_weight": 1000, "netto_weight": 520, "spool_weight": 260};type=application/json' \
  -F image=@./sample_images/filament19.jpg

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 19.99, "color_name": "Orange", "color_hex": "#ff6a13", "original_weight": 1000, "netto_weight": 955, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament21.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 19.99, "color_name": "Pink", "color_hex": "#f55a74", "original_weight": 1000, "netto_weight": 985, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament22.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 19.99, "color_name": "Brown", "color_hex": "#9c432c", "original_weight": 1000, "netto_weight": 618, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament23.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 1, "price": 19.99, "color_name": "Beige", "color_hex": "#f7e6de", "original_weight": 1000, "netto_weight": 918, "spool_weight": 250};type=application/json' \
  -F image=@./sample_images/filament24.png

curl -X POST "$BASE_URL/filaments-with-image" \
  -F 'json={"id_product": 13, "price": 9.99, "color_name": "Yellow", "color_hex": "#fefe01", "original_weight": 1000, "netto_weight": 853, "spool_weight": 137};type=application/json' \
  -F image=@./sample_images/filament20.png
