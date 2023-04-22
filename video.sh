cargo run --release
ffmpeg -framerate 24 -i %03d.png -pix_fmt yuv420p out.mp4 -b:v 10M -y
echo 
