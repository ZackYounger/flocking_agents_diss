# Put auxiliary files in a separate directory
$aux_dir = 'build';
$out_dir = 'build';

# Use pdflatex
$pdf_mode = 1;

# Enable synctex for editor integration
$pdflatex = 'pdflatex -synctex=1 -interaction=nonstopmode -file-line-error %O %S';
