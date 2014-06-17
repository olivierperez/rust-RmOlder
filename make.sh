mkdir out
mkdir out_test

echo "============================================="
echo "----- Compile : test"
rustc rmolder.rs --test --out-dir out_test
if [ $? != 0 ]
then
	exit 1
fi

echo "----- Execute : test"
./out_test/rmolder
if [ $? != 0 ]
then
	exit 1
fi

echo "----- Execute : rustdoc"
rustdoc rmolder.rs
echo "done"
if [ $? != 0 ]
then
	exit 1
fi

echo "============================================="
echo "----- Compile : rmolder"
rustc --crate-type=lib --out-dir out rmolder.rs

echo "----- Compile : main"
rustc --out-dir out main.rs -L out

echo "----- Execute : main"
./out/main -d test -a 1000000 --dry

