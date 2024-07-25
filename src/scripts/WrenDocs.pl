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
    
    my $output = "";
    my @new_source = $source =~ /\/{3}.*/g;
    my $module = "";
    my @all_classes = ();
    foreach(@new_source) {
        my @ident_name = ($_ =~ /(?<=\{)(.*)(?=\})/g);
        
        if(@ident_name) {
            if($ident_name[0] eq "class") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $output .= "## $ident_sig\n";
                push(@all_classes, "> - [$ident_sig]($ident_sig)");
            } elsif($ident_name[0] eq "module") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $module .= "# $ident_sig\n";
            } elsif($ident_name[0] eq "static setter" || $ident_name[0] eq "setter") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $output .= "### ``$ident_sig``\n";
                $output .= "$ident_name[0]\n";
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
                    $output .= "$ident_name[0] with arity($w_count) and returns $returns\n";
                } else {
                    $output .= "$ident_name[0] returns $returns\n";
                }
            }
        } else {
            my @descr = ($_ =~ /[^\/].*/g);
            $output .= "> $descr[0]\n";
        }
    }
    
    print $output;
    ##close($fh);
    return $module."### Classes\n".join("\n", @all_classes)."\n".$output;
}
                                       
my $filename = 'src/scripts/WrenDocs.md';
unlink($filename);
open(my $fh, '>>', $filename) or die "Could not open file '$filename' $!";
print $fh create_doc("src/scripts/game.wren");
close $fh;
print "done\n";