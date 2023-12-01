#!/bin/sh


TEMPLATE="pub fn sove() {\n\
  println!(\"Solving Year 2015, Day01\");\n\
}\n"

OUTPUT_DIR="src/puzzles"

mkdir -p $OUTPUT_DIR/year{2015..2023}

MOD_YEARS=""

# Loop through years (2015 to 2023)
for year in {2015..2023}; do

  MOD_DAYS=""
  # Loop through days (01 to 25)
  for day in {01..25}; do
    # Output file
    day_with_zero=$(printf "%02d" $day)
    output_file="$OUTPUT_DIR/year${year}/day${day_with_zero}.rs"

    # Replace placeholders and save to the output file
    echo "$TEMPLATE" | sed -e "s/{YEAR}/$year/g; s/{DAY}/$day_with_zero/g" > "$output_file"

    # Concat day module names
    MOD_DAYS+="mod day${day_with_zero};\n"
  done

  # Generate mod.rs file for the year directory
  echo "$MOD_DAYS" >> "$OUTPUT_DIR/year${year}/mod.rs"

  # Concat year module names
  MOD_YEARS+="mod year${year};\n"
done

# Generate mod.rs file for the root directory
echo "$MOD_YEARS" >> "$OUTPUT_DIR/mod.rs"