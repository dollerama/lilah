#! /usr/bin/perl
use strict;
use warnings;

sub create_doc {
    my $file = $_[0];
    
    my $source = do {
        local $/ = undef;
        open my $fh, "<", $file
            or die "could not open $file: $!";
        <$fh>;
    };
    
    my $output_header = "";
    my $output = "";
    my $output_final = "";
    my @new_source = $source =~ /\/{3}.*/g;
    my $module = "";
    my @all_classes = ();
    my @all_in_class = ();
    my $prev_class = 0;
    foreach(@new_source) {
        my @ident_name = ($_ =~ /(?<=\{)(.*)(?=\})/g);
        
        if(@ident_name) {
            if($ident_name[0] eq "class") {
                if(@all_in_class-0 == 0) {
                     $output_final .= $output_header.$output;
                } elsif(@all_in_class-0 == 1) {
                    $output_final .= $output_header."@all_in_class"."\n".$output;
                } else {
                    $output_final .= $output_header.join("\n", @all_in_class)."\n".$output;
                }
                $output = "";
                $output_header = "";

                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $output_header .= "## $ident_sig\n";
                my $ident_sig_lc = lc($ident_sig);
                push(@all_classes, "> - [$ident_sig](##$ident_sig_lc)");
                @all_in_class = ();
                $prev_class = 1;
            } elsif($ident_name[0] eq "module") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $module .= "# $ident_sig\n";
                $prev_class = 0;
            } elsif($ident_name[0] eq "static setter" || $ident_name[0] eq "setter") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $output .= "### ``$ident_sig``\n";
                $output .= "$ident_name[0]\n";

                push(@all_in_class, "> - $ident_sig");
                $prev_class = 0;
            } else {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)(?=(\s->))/g)[1];
                my @words = ($ident_sig =~ /(\w+)/g);
                @words = @words[1..$#words];
                my $w_count = (@words-0);
                if($w_count != 1) {
                    $w_count /= 2;
                }
                my $returns = ($_ =~ /(?<=(->\W))(.*)/g)[1];
                $output .= "### ``$ident_sig``\n";

                if($w_count != 0) {
                    $output .= "$ident_name[0] with arity($w_count) and returns ``$returns``\n";
                } else {
                    $output .= "$ident_name[0] returns ``$returns``\n";
                }

                push(@all_in_class, "> - $ident_sig");
                $prev_class = 0;
            }
        } else {
            my @descr = ($_ =~ /[^\/].*/g);

            $output .= "> $descr[0]\n";

            $prev_class = 0;
        }
    }

    my $result .= $module."### Classes\n".join("\n", @all_classes)."\n".$output_final;
    if(@all_in_class-0 == 1) {
        $result .= $output_header."@all_in_class"."\n".$output;
    } elsif(@all_in_class-0 == 0) {
        $result .= $output_header.$output;
    } else {
        $result .= $output_header.join("\n", @all_in_class)."\n".$output;
    }
    ##print $result;
    ##close($fh);
    return $result;
}
                                       
my $filename = 'src/scripts/WrenDocs.md';
unlink($filename);
open(my $fh, '>>', $filename) or die "Could not open file '$filename' $!";
print $fh create_doc("src/scripts/game.wren");
close $fh;
print "done\n";