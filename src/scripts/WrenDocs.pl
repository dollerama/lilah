#! /usr/bin/perl
use strict;
use File::Basename;
use warnings;
use Getopt::Long qw(GetOptions);
use Data::Dumper;

sub create_docv2 {
    my $file = $_[0];
    my $verbose = $_[1];

    if($verbose) {
      print "  WrenDoc: starting ".basename($file)."\n";
    }
    
    my $source = do {
        local $/ = undef;
        open my $fh, "<", $file
            or die "WrenDoc: could not open $file: $!";
        <$fh>;
    };

    my @new_source = $source =~ /.*/g;


    my $parameters;
    my $return = "_";
    my $description = "";

    my $result = "";
    my $header = ""; 
    my $final = "";
    my $depth = 0;

    my @methods = ();
    my @classes = ();

    foreach(@new_source) {
        if($_ =~ m/^\s*\/\/.*/) {
          next;
        }

        if(($_ =~ /\/{3}.*/g) != 0) { 
            if(($_ =~ /(?<=\/{3})(.*)(?=(\s->))/s) == 0) {
                $description .= "> ".($_ =~ /(?<=\/{3})(.*)/s)[0]."\n";
            } else {
                $description = "";
                $parameters = ($_ =~ /(?<=\/{3})(.*)(?=(\s->))/s)[0];
                $return = ($_ =~ /(?<=(->\W))(.*)/s)[1];
            }
        }
        
        if(($_ =~ /^\s*foreign/s) != 0) {
            if(($_ =~ /class/s) != 0) {
                if($verbose) {
                    print "? foreign class\n";
                }

                my $class_inherit = ($_ =~ /(?<=(is\s))(.*)(?=(\s\{))/g)[1];
                
                if($class_inherit ne "") {
                    my $class_name = ($_ =~ /(?<=(class\s))(.*)(?=(\s[is]))/g)[1];
                    $header .= "### Foreign Class ``".$class_name."``\n";
                    $header .= "> Inherits from ``".$class_inherit."``\n";
                    push(@classes, "> - [".$class_name."](#".lc($class_name).")");
                } else {
                    my $class_name = ($_ =~ /(?<=(class\s))(.*)(?=(\s[{]))/g)[1];
                    $header .= "### Foreign Class ``".$class_name."``\n";
                    push(@classes, "> - [".$class_name."](#".lc($class_name).")");
                }
            }
        } else {
            if(($_ =~ /^\s*class/s) != 0) {
                if($verbose) {
                  print "? class\n";
                }

                my $class_inherit = ($_ =~ /(?<=(is\s))(.*)(?=(\s\{))/g)[1];
                
                if($class_inherit ne "") {
                    my $class_name = ($_ =~ /(?<=(class\s))(.*)(?=(\s[is]))/g)[1];
                    $header .= "### Class ``".$class_name."``\n";
                    $header .= "> Inherits from ``".$class_inherit."``\n";
                    push(@classes, "> - [".$class_name."](#".lc($class_name).")");
                } else {
                    my $class_name = ($_ =~ /(?<=(class\s))(.*)(?=(\s[{]))/g)[1];
                    $header .= "### Class ``".$class_name."``\n";
                    push(@classes, "> - [".$class_name."](#".lc($class_name).")");
                }
            }
            if($depth == 1) {
                if(($_ =~ /static/s) != 0) {
                    my @beginning = ($_ =~ /(.*)(?<={)/s);
                    
                    if(@beginning != 0) {
                        if(join("", @beginning) =~ m/=\s?\(/) {
                            if($verbose) {
                                print "? static setter\n";
                            }

                            my $name = ($_ =~ /(?<=(static\s))(.*)(?=(=\s?\())/s)[1];
                            $name =~ s/^\s+|\s+$//g;
                            
                            my @param = split(", ", ($_ =~ /(?<=(\())(.*)(?=(\)))/s)[1]);
                            my @param_t = split(", ", $parameters);
        
                            $result .= "##### Static Setter ``".$name;
                            $result .= " = ";
                            for( $a = 0; $a <= $#param; $a = $a + 1 ) {
                                if($param_t[$a] eq "") {
                                    $result .= $param[$a].": _";
                                } else {
                                    $result .= $param[$a].": ".$param_t[$a];
                                }
                                if($a != $#param) {
                                    $result .= ", ";
                                }
                            }
                            $result .= "``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        }
                        elsif(join("", @beginning) =~ m/\(/) {
                            if($verbose) {
                                print "? static method\n";
                            }

                            my $name = ($_ =~ /(?<=(static\s))(.*)(?=\()/s)[1];
                            my @param = split(", ", ($_ =~ /(?<=(\())(.*)(?=(\)))/s)[1]);
                            my @param_t = split(", ", $parameters);
                
                            $result .= "##### Static Method ``".$name; 
                            $result .= "(";
                            for( $a = 0; $a <= $#param; $a = $a + 1 ) {
                                if($param_t[$a] eq "") {
                                    $result .= $param[$a].": _";
                                } else {
                                    $result .= $param[$a].": ".$param_t[$a];
                                }
                                if($a != $#param) {
                                    $result .= ", ";
                                }
                            }
                            $result .= ")``\n"."``return ".$return."``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        } else {
                            if($verbose) {
                                print "? static getter\n";
                            }              
                            my $name = ($_ =~ /(?<=(static\s))(.*)(?=(\{))/s)[1];
                            $name =~ s/^\s+|\s+$//g;
        
                            $result .= "##### Static Getter ``".$name; 
                            $result .= $parameters;
                            $result .= "``\n"."``return ".$return."``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        }
                    }
                } elsif(($_ =~ /construct/s) != 0) {
                    if($verbose) {
                        print "? constructor\n";
                    }    
                    my $name = ($_ =~ /(?<=(construct\s))(.*)(?=\()/s)[1];
                    my @param = split(", ", ($_ =~ /(?<=(\())(.*)(?=(\)))/s)[1]);
                    my @param_t = split(", ", $parameters);
        
                    $result .= "##### Constructor ``".$name; 
                    $result .= "(";
                    for( $a = 0; $a <= $#param; $a = $a + 1 ) {
                        if($param_t[$a] eq "") {
                            $result .= $param[$a].": _";
                        } else {
                            $result .= $param[$a].": ".$param_t[$a];
                        }
                        if($a != $#param) {
                            $result .= ", ";
                        }
                    }
                    $result .= ")``\n"."``return ".$return."``\n";
                    $result .= $description."\n";
                    push(@methods, "> - ".$name);
                } else {
                    my @beginning = ($_ =~ /(.*)(?<={)/s);
                    
                    if(@beginning != 0) {
                        if(join("", @beginning) =~ m/=\s?\(/) {
                            if($verbose) {
                                print "? setter\n";
                            }            
                            my $name = ($_ =~ /(.*)(?=(=\s?\())/s)[0];
                            $name =~ s/^\s+|\s+$//g;
                            
                            my @param = split(", ", ($_ =~ /(?<=(\())(.*)(?=(\)\s\{))/s)[1]);
                            my @param_t = split(", ", $parameters);
        
                            $result .= "##### Setter ``".$name; 
                            $result .= " = ";
                            for( $a = 0; $a <= $#param; $a = $a + 1 ) {
                                if($param_t[$a] eq "") {
                                    $result .= $param[$a].": _";
                                } else {
                                    $result .= $param[$a].": ".$param_t[$a];
                                }
                                if($a != $#param) {
                                    $result .= ", ";
                                }
                            }
                            $result .= "``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        }
                        elsif(join("", @beginning) =~ m/\(/) {
                            if($verbose) {
                                print "? method\n";
                            }            
                            my $name = ($_ =~ /(.*)(?=\()/s)[0];
                            $name =~ s/^\s+|\s+$//g;
                            my @param = split(", ", ($_ =~ /(?<=(\())(.*)(?=(\)))/s)[1]);
                            my @param_t = split(", ", $parameters);
                
                            $result .= "##### Method ``".$name; 
                            $result .= "(";
                            for( $a = 0; $a <= $#param; $a = $a + 1 ) {
                                if($param_t[$a] eq "") {
                                    $result .= $param[$a].": _";
                                } else {
                                    $result .= $param[$a].": ".$param_t[$a];
                                }
                                if($a != $#param) {
                                    $result .= ", ";
                                }
                            }
                            $result .= ")``\n"."``return ".$return."``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        }
                        else {
                            if($verbose) {
                                print "? getter\n";
                            }            
                            my $name = ($_ =~ /(.*)(?=(\{))/s)[0];
                            $name =~ s/^\s+|\s+$//g;
        
                            $result .= "##### Getter ``".$name; 
                            $result .= $parameters;
                            $result .= "``\n"."``return ".$return."``\n";
                            $result .= $description."\n";
                            push(@methods, "> - ".$name);
                        }
                    }
                }
            }
        }
        
        if($_ =~ m/{(?=(?:(?:[^"]*"){2})*[^"]*$)/) {
            $depth = $depth+1;
            $description = "";
            $parameters = "";
            $return = "_";
        }
        if($_ =~ m/}(?=(?:(?:[^"]*"){2})*[^"]*$)/) {
            $depth = $depth-1;
            $description = "";
            $parameters = "";
            $return = "_";
            if($depth == 0) {
                $final .= $header."#### Methods\n".join("\n", @methods)."\n".$result;
                $result = "";
                $header = "";
                @methods = ();
            }
        }
    }

    my $output = "## Module ``".basename($file, ".wren")."``\n### Classes\n".join("\n", @classes)."\n".$final;

    return(basename($file, ".wren"), $output);
}

sub create_doc {
    my $file = $_[0];
    my $verbose = $_[1];

    if($verbose) {
      print "  WrenDoc: starting ".basename($file)."\n";
    }
    
    my $source = do {
        local $/ = undef;
        open my $fh, "<", $file
            or die "WrenDoc: could not open $file: $!";
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

                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)(?=(\s:))/g)[1] || ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                my $ident_inherit = ($_ =~ /(?<=(:\s))(.*)/g)[1] || "";
                if($ident_inherit ne "") {
                    $ident_inherit = "\nInherits from ``$ident_inherit``";
                }
                $output_header .= "## $ident_sig$ident_inherit\n";

                my $ident_sig_lc = lc($ident_sig);
                push(@all_classes, "> - [$ident_sig](#$ident_sig_lc)");
                @all_in_class = ();
                $prev_class = 1;


                if($verbose) {
                  print "    ".basename($file).": class => ".$ident_sig."\n";
                }
            } elsif($ident_name[0] eq "module") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $module .= "$ident_sig";
                $prev_class = 0;
            } elsif($ident_name[0] eq "static setter" || $ident_name[0] eq "setter") {
                my $ident_sig = ($_ =~ /(?<=(\}\s))(.*)/g)[1];
                $output .= "### ``$ident_sig``\n";
                $output .= "$ident_name[0]\n";

                push(@all_in_class, "> - $ident_sig");
                $prev_class = 0;


                if($verbose) {
                  print "      ".basename($file).": setter => ".$ident_sig."\n";
                }
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
                
                if($verbose) {
                  print "      ".basename($file).": ".$ident_name[0]." => ".$ident_sig."\n";
                }
            }
        } else {
            my @descr = ($_ =~ /[^\/].*/g);

            $output .= "> $descr[0]\n";

            $prev_class = 0;
            
            if($verbose) {
              print "      ".basename($file).": description => ".@descr."\n";
            }
        }
    }

    my $result .= "# ".$module."\n### Classes\n".join("\n", @all_classes)."\n".$output_final;
    if(@all_in_class-0 == 1) {
        $result .= $output_header."@all_in_class"."\n".$output;
    } elsif(@all_in_class-0 == 0) {
        $result .= $output_header.$output;
    } else {
        $result .= $output_header.join("\n", @all_in_class)."\n".$output;
    }
    return ($module, $result);
}
                                       
my @inputs;
my $outputs;
my $verbose;
GetOptions('out|o=s' => \$outputs, 'in|i=s' => \@inputs, 'verbose|v' => \$verbose) or die 'Usage: $0 -in|o FILE.wren -out|o FILE.md -verbose|v\n';
@inputs = split(/,/,join(',',@inputs));

unlink($outputs);

my $file_header = "# ".basename($outputs, ".md")."\n"."### Modules\n";
my $file = "";

print "WrenDoc: building ".basename($outputs)."\n";

for my $i (0 .. $#inputs) {
  my $in  = $inputs[$i];
  my @docu = create_docv2($in, $verbose);
  my $file_header_proc = "module---".lc($docu[0])."--";
  #$file_header_proc =~ s/[^a-zA-Z]+/-/g;
  $file_header .= "> - [".$docu[0]."](#".$file_header_proc.")\n";
  $file .= $docu[1];
  print "  WrenDoc: built doc => ".basename($in)."\n";
}

open(my $fh, '>>', $outputs) or die 'WrenDoc Err: Could not open file '.$outputs.' $!';
print $fh $file_header.$file; 
close $fh;
print "WrenDoc: Finished\n";
