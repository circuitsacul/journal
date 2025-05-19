alias tw := build-tailwind

build-tailwind *args:
    npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css {{args}}
