# Generate Ts-EEL locally which should later be uploaded to release assets

list=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

for item in "${list[@]}"; do
    deno compile --allow-all --output ts-eel/assets/"${item}" --target $item ts-eel/main.ts
    pushd ts-eel/assets/
    tar -czvf "ts-eel-${item}.tar.gz" "$item"
    rm "$item"
    popd
done

open ts-eel/assets/
