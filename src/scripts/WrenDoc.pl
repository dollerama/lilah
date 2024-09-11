#! /usr/bin/perl
use strict;
use File::Basename;
use warnings;
use Getopt::Long qw(GetOptions);
use Data::Dumper;

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

    my @new_source = $source =~ /.*/g;


    my $parameters;
    my $return = "_";
    my $description = "";

    my $result = "";
    my $header = ""; 
    my $final = "";
    my $depth = 0;

    my $mod = lc(basename($file, ".wren"));
    my $mod_unf = basename($file, ".wren");
    my @classes = ();
    my @methods = ();
    my @getters = ();
    my @setters = ();
    my @constructors = ();

    foreach (@new_source) {

        if ( ( $_ =~ /\/{3}.*/g ) != 0 ) {
            if ( ( $_ =~ /(?<=\/{3})(.*)(?=(\s->))/s ) == 0 ) {
                $description .= "> " . ( $_ =~ /(?<=\/{3})(.*)/s )[0] . "\n";
            }
            else {
                $description = "";
                $parameters  = ( $_ =~ /(?<=\/{3})(.*)(?=(\s->))/s )[0];
                $parameters =~ s/^\s+|\s+$//g;
                if($parameters eq "_") {
                    $parameters = "";
                }
                $return      = ( $_ =~ /(?<=(->\W))(.*)/s )[1];
            }

            next;
        }

        if ( $_ =~ m/^\s*\/\/.*/ ) {
            next;
        }

        if ( ( $_ =~ /^\s*foreign/s ) != 0 ) {
            if ( ( $_ =~ /class/s ) != 0 ) {
                if ($verbose) {
                    print "    WrenDoc Found: foreign class\n";
                }

                my $class_inherit = ( $_ =~ /(?<=(is\s))(.*?)(?=(\s\{))/g )[1];

                if ($class_inherit) {
                    my $class_name =
                      ( $_ =~ /(?<=(class\s))(.*)(?=(\s[is]))/g )[1];
                    my $link = $mod."-"."-k".$#classes;
                    $header .= "### Foreign Class ``" . $class_name . "`` <a id='".$link."'></a> \n";
                    $header .= "> Inherits from ``" . $class_inherit . "``\n";
                    $header .= ">\n".$description . "\n";
                    push( @classes,
                        "> - [" . $class_name . "](#" .$link. ")" );
                        
                    $description = "";
                    $parameters  = "";
                    $return      = "_";
                }
                else {
                    my $class_name =
                      ( $_ =~ /(?<=(class\s))(.*?)(?=(\s[{]))/g )[1];
                    my $link = $mod."-"."-k".$#classes;
                    $header .= "### Foreign Class ``" . $class_name . "`` <a id='" . $link . "'></a> \n";
                    $header .= ">\n".$description . "\n";
                    push( @classes,
                        "> - [" . $class_name . "](#" . $link . ")" );
                        
                    $description = "";
                    $parameters  = "";
                    $return      = "_";
                }
            }

            if ( $depth == 1 ) {
                if ( ( $_ =~ /static/s ) != 0 ) {
                    my @beginning = ( $_ =~ /(.*?)(?<=\()/s );

                    
                    if ( join( "", @beginning ) =~ m/=\s?\(/ ) {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign static setter\n";
                        }

                        my $name =
                          ( $_ =~ /(?<=(static\s))(.*?)(?=(=\s?\())/s )[1];
                        $name =~ s/^\s+|\s+$//g;

                        my @param =
                          split( ", ", ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                        my @param_t = split( ", ", $parameters );

                        $result .= "##### Foreign Static Setter ``" . $name;
                        if($name =~ m/\[/) {
                          $result .= ": ".$param_t[0];
                          $result .= " = ".$param[0].": ".$param_t[1];
                        } else {
                            
                            $result .= " = ";
                            for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                if ( !@param_t || $param_t[$a] eq "" ) {
                                    $result .= $param[$a] . ": _";
                                }
                                else {
                                    $result .= $param[$a] . ": " . $param_t[$a];
                                }
                                if ( $a != $#param ) {
                                    $result .= ", ";
                                }
                            }
                        }
                        my $link = $mod."-".$#classes."-s".$#setters;
                        $result .= "`` <a id='".$link."'></a>\n";
                        $result .= $description . "\n";
                        push( @setters, "> - [" . $name . "](#".$link.")" );
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    }
                    elsif ( join( "", @beginning ) =~ m/\(/ ) {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign static method\n";
                        }

                        my $name = ( $_ =~ /(?<=(static\s))(.*?)(?=\()/s )[1];
                        my @param =
                          split( ", ", ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                        my @param_t = split( ", ", $parameters );

                        $result .= "##### Foreign Static Method ``" . $name;
                        $result .= "(";
                        for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                            if ( !@param_t || $param_t[$a] eq "" ) {
                                $result .= $param[$a] . ": _";
                            }
                            else {
                                $result .= $param[$a] . ": " . $param_t[$a];
                            }
                            if ( $a != $#param ) {
                                $result .= ", ";
                            }
                        }
                        my $link = $mod."-".$#classes."-m".$#methods;
                        $result .= ")`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                        $result .= $description . "\n";
                        push( @methods, "> - [" . $name . "](#".$link.")" );
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    }
                    else {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign static getter\n";
                        }
                        my $name =
                          ( $_ =~ /(?<=(foreign static\s))(.*?)/s )[1];
                        $name =~ s/^\s+|\s+$//g;

                        $result .= "##### Foreign Static Getter ``" . $name;
                        if($parameters) {
                            $result .= ": ".$parameters;
                        }
                        my $link = $mod."-".$#classes."-g".$#getters;
                        $result .= "`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                        $result .= $description . "\n";
                        push( @getters, "> - [" . $name . "](#".$link.")" );
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    }
                }
                else {
                    my @beginning = ( $_ =~ /(.*?)(?<=\))/s );

                    if ( join( "", @beginning ) =~ m/=\s?\(/ ) {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign setter\n";
                        }

                        my $name =
                          ( $_ =~ /(?<=(foreign\s))(.*?)(?=(=\s?\())/s )[1];
                        $name =~ s/^\s+|\s+$//g;

                        my @param =
                          split( ", ", ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                        my @param_t = split( ", ", $parameters );

                        $result .= "##### Foreign Setter ``" . $name;

                        if($name =~ m/\[/) {
                          $result .= ": ".$param_t[0];
                          $result .= " = ".$param[0].": ".$param_t[1];
                        } else {
                            $result .= " = ";
                            for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                if ( !@param_t || $param_t[$a] eq "" ) {
                                    $result .= $param[$a] . ": _";
                                }
                                else {
                                    $result .= $param[$a] . ": " . $param_t[$a];
                                }
                                if ( $a != $#param ) {
                                    $result .= ", ";
                                }
                            }
                        }
                        my $link = $mod."-".$#classes."-s".$#setters;
                        $result .= "`` <a id='".$link."'></a>\n";
                        $result .= $description . "\n";
                        push( @setters, "> - [" . $name . "](#".$link.")");
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    } elsif ( join( "", @beginning ) =~ m/\(/ )  {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign method\n";
                        }
                        my $name = ( $_ =~ /(?<=foreign)(.*?)(?=\()/s )[0];
                        $name =~ s/^\s+|\s+$//g;
                        my @param =
                          split( ", ", ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                        my @param_t = split( ", ", $parameters );

                        $result .= "##### Foreign Method ``" . $name;
                        $result .= "(";
                        for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                            if ( !@param_t || $param_t[$a] eq "" ) {
                                $result .= $param[$a] . ": _";
                            }
                            else {
                                $result .= $param[$a] . ": " . $param_t[$a];
                            }
                            if ( $a != $#param ) {
                                $result .= ", ";
                            }
                        }
                        my $link = $mod."-".$#classes."-m".$#methods;
                        $result .= ")`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                        $result .= $description . "\n";
                        push( @methods, "> - [" . $name . "](#".$link.")" );
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    } else {
                        if ($verbose) {
                            print "    WrenDoc Found: foreign getter\n";
                        }
                        my $name = ( $_ =~ /(?<=foreign)(.*?)/s )[0];
                        $name =~ s/^\s+|\s+$//g;

                        $result .= "##### Foreign Getter ``" . $name;
                        if($parameters) {
                            $result .= ": ".$parameters;
                        }
                        my $link = $mod."-".$#classes."-g".$#getters;
                        $result .= "`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                        $result .= $description . "\n";
                        push( @getters, "> - [" . $name . "](#".$link.")");
                        
                        $description = "";
                        $parameters  = "";
                        $return      = "_";
                    }
                }
            }
        }
        else {
            if ( ( $_ =~ /^\s*class/s ) != 0 ) {
                if ($verbose) {
                    print "    WrenDoc Found: class\n";
                }

                my $class_inherit = ( $_ =~ /(?<=(is\s))(.*?)(?=(\s\{))/g )[1];

                if ($class_inherit) {
                    my $class_name =
                      ( $_ =~ /(?<=(class\s))(.*?)(?=(\s[is]))/g )[1];
                    my $link = $mod."-"."-k".$#classes;
                    $header .= "### Class ``" . $class_name . "`` <a id='".$link."'></a> \n";
                    $header .=
                      "> Inherits from ``" . $class_inherit . "``\n";
                    $header .= ">\n".$description . "\n";
                    push( @classes,
                            "> - [". $class_name . "](#".$link.")" );
                    
                    $description = "";
                    $parameters  = "";
                    $return      = "_";
                }
                else {
                    my $class_name =
                      ( $_ =~ /(?<=(class\s))(.*?)(?=(\s[{]))/g )[1];
                    my $link = $mod."-"."-k".$#classes;
                    $header .= "### Class ``" . $class_name . "`` <a id='".$link."'></a>\n";
                    $header .= ">\n".$description . "\n";
                    push( @classes,
                        "> - [" . $class_name . "](#" . $link . ")" );
                        
                    $description = "";
                    $parameters  = "";
                    $return      = "_";
                }
            }

            if ( $depth == 1 ) {
                if ( ( $_ =~ /static/s ) != 0 ) {
                    my @beginning = ( $_ =~ /(.*?)(?<={)/s );

                    if ( @beginning != 0 ) {
                        if ( join( "", @beginning ) =~ m/=\s?\(/ ) {
                            if ($verbose) {
                                print "    WrenDoc Found: static setter\n";
                            }

                            my $name =
                              ( $_ =~ /(?<=(static\s))(.*?)(?=(=\s?\())/s )[1];
                            $name =~ s/^\s+|\s+$//g;

                            my @param = split( ", ",
                                ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                            my @param_t = split( ", ", $parameters );

                            $result .= "##### Static Setter ``" . $name;
                            if($name =~ m/\[/) {
                              $result .= ": ".$param_t[0];
                              $result .= " = ".$param[0].": ".$param_t[1];
                            } else {
                                $result .= " = ";
                                for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                    if ( !@param_t || $param_t[$a] eq "" ) {
                                        $result .= $param[$a] . ": _";
                                    }
                                    else {
                                        $result .= $param[$a] . ": " . $param_t[$a];
                                    }
                                    if ( $a != $#param ) {
                                        $result .= ", ";
                                    }
                                }
                            }
                            my $link = $mod."-".$#classes."-s".$#setters;
                            $result .= "`` <a id='".$link."'></a>\n";
                            $result .= $description . "\n";
                            push( @setters, "> - [" . $name . "](#".$link.")" );
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                        }
                        elsif ( join( "", @beginning ) =~ m/\(/ ) {
                            if ($verbose) {
                                print "    WrenDoc Found: static method\n";
                            }

                            my $name =
                              ( $_ =~ /(?<=(static\s))(.*?)(?=\()/s )[1];
                            my @param = split( ", ",
                                ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                            my @param_t = split( ", ", $parameters );

                            $result .= "##### Static Method ``" . $name;
                            $result .= "(";
                            for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                if ( !@param_t || $param_t[$a] eq "" ) {
                                    $result .= $param[$a] . ": _";
                                }
                                else {
                                    $result .= $param[$a] . ": " . $param_t[$a];
                                }
                                if ( $a != $#param ) {
                                    $result .= ", ";
                                }
                            }
                            my $link = $mod."-".$#classes."-m".$#methods;
                            $result .= ")`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                            $result .= $description . "\n";
                            push( @methods, "> - [" . $name . "](#".$link.")" );
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                        }
                        else {
                            if ($verbose) {
                                print "    WrenDoc Found: static getter\n";
                            }
                            my $name =
                              ( $_ =~ /(?<=(static\s))(.*?)(?=(\{))/s )[1];
                            $name =~ s/^\s+|\s+$//g;

                            $result .= "##### Static Getter ``" . $name;
                            if($parameters) {
                                $result .= ": ".$parameters;
                            }
                            my $link = $mod."-".$#classes."-g".$#getters;
                            $result .= "`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                            $result .= $description . "\n";
                            push( @getters, "> - [" . $name . "](#".$link.")");
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                        }
                    }
                }
                elsif ( ( $_ =~ /construct/s ) != 0 ) {
                    if ($verbose) {
                        print "    WrenDoc Found: constructor\n";
                    }
                    my $name = ( $_ =~ /(?<=(construct\s))(.*?)(?=\()/s )[1];
                    my @param =
                      split( ", ", ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                    my @param_t = split( ", ", $parameters );
                    
                    $result .= "##### Constructor ``" . $name;
                    $result .= "(";
                    for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                        if ( !@param_t || $param_t[$a] eq "" ) {
                            $result .= $param[$a] . ": _";
                        }
                        else {
                            $result .= $param[$a] . ": " . $param_t[$a];
                        }
                        if ( $a != $#param ) {
                            $result .= ", ";
                        }
                    }
                    my $link = $mod."-".$#classes."-c".$#constructors;
                    $result .= ")`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                    $result .= $description . "\n";
                    push( @constructors, "> - [" . $name . "](#".$link.")" );
                    
                    $description = "";
                    $parameters  = "";
                    $return      = "_";
                }
                else {
                    my @beginning = ( $_ =~ /(.*?)(?<={)/s );

                    if ( @beginning != 0 ) {
                        if ( join( "", @beginning ) =~ m/=\s?\(/ ) {
                            if ($verbose) {
                                print "    WrenDoc Found: setter\n";
                            }
                            my $name = ( $_ =~ /(.*?)(?=(=\s?\())/s )[0];
                            $name =~ s/^\s+|\s+$//g;

                            my @param = split( ", ",
                                ( $_ =~ /(?<=(\())(.*?)(?=(\)\s\{))/s )[1] );
                            my @param_t = split( ", ", $parameters );

                            if ( $name eq "=" ) {
                                $result .= "##### Method ``==(";
                            }
                            else {
                                $result .= "##### Setter ``" . $name;
                            }

                            if($name =~ m/\[/) {
                              if(@param_t) {
                                  $result .= ": ".$param_t[0];
                                  $result .= " = ".$param[0].": ".$param_t[1];
                              }
                            } else {
                                $result .= " = ";
                                for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                    if ( !@param_t || $param_t[$a] eq "" ) {
                                        $result .= $param[$a] . ": _";
                                    }
                                    else {
                                        $result .= $param[$a] . ": " . $param_t[$a];
                                    }
                                    if ( $a != $#param ) {
                                        $result .= ", ";
                                    }
                                }
                            }
                            
                            if ( $name eq "=" ) {
                                my $link = $mod."-".$#classes."-m".$#methods;
                                $result .= ")`` <a id='".$link."'></a>\n";
                                
                                push( @methods, "> - [" . $name . "](#".$link.")");
                            }
                            else {
                                my $link = $mod."-".$#classes."-s".$#setters;
                                $result .= "`` <a id='".$link."'></a>\n";
                                push( @setters, "> - [" . $name . "](#".$link.")");
                            }
                            $result .= $description . "\n";
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                            
                        }
                        elsif ( join( "", @beginning ) =~ m/\(/ ) {
                            if ($verbose) {
                                print "    WrenDoc Found: method\n";
                            }
                            my $name = ( $_ =~ /(.*?)(?=\()/s )[0];
                            $name =~ s/^\s+|\s+$//g;
                            my @param = split( ", ",
                                ( $_ =~ /(?<=(\())(.*?)(?=(\)))/s )[1] );
                            my @param_t = split( ", ", $parameters );

                            $result .= "##### Method ``" . $name;
                            $result .= "(";
                            for ( $a = 0 ; $a <= $#param ; $a = $a + 1 ) {
                                if ( !@param_t || $param_t[$a] eq "" ) {
                                    $result .= $param[$a] . ": _";
                                }
                                else {
                                    $result .= $param[$a] . ": " . $param_t[$a];
                                }
                                if ( $a != $#param ) {
                                    $result .= ", ";
                                }
                            }
                            my $link = $mod."-".$#classes."-m".$#methods;
                            $result .= ")`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                            $result .= $description . "\n";
                            push( @methods, "> - [" . $name . "](#".$link.")" );
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                        }
                        else {
                            if ($verbose) {
                                print "    WrenDoc Found: getter\n";
                            }
                            my $name = ( $_ =~ /(.*?)(?=(\{))/s )[0];
                            $name =~ s/^\s+|\s+$//g;
                            
                            $result .= "##### Getter ``" . $name;
                            if($parameters) {
                                $result .= ": ".$parameters;
                            }
                            my $link = $mod."-".$#classes."-g".$#getters;
                            $result .= "`` <a id='".$link."'></a>\n" . "``return " . $return . "``\n";
                            $result .= $description . "\n";
                            push( @getters, "> - [" . $name . "](#".$link.")");
                            
                            $description = "";
                            $parameters  = "";
                            $return      = "_";
                        }
                    }
                }
            }
        }

        if ( $_ =~ m/{(?=(?:(?:[^"]*"){2})*[^"]*$)/ ) {
            $depth       = $depth + 1;
        }
        if ( $_ =~ m/}(?=(?:(?:[^"]*"){2})*[^"]*$)/ ) {
            $depth       = $depth - 1;
            if ( $depth == 0 ) {
                $final .= $header;
                if(@constructors) {
                    $final .= "#### Constructors\n"
                    . join( "\n", @constructors ) . "\n";
                }
                if(@getters) {
                    $final .= "#### Getters\n"
                    . join( "\n", @getters ) . "\n";
                }
                if(@setters) {
                    $final .= "#### Setters\n"
                    . join( "\n", @setters ) . "\n";
                }
                if(@methods) {
                    $final .= "#### Methods\n"
                    . join( "\n", @methods ) . "\n";
                }
                $final .= $result;
                $result  = "";
                $header  = "";
                @methods = ();
                @constructors = ();
                @getters = ();
                @setters = ();
            }
        }
    }

    my $output = "## Module ``".$mod_unf."``\n### Classes\n".join("\n", @classes)."\n".$final;

    return($mod_unf, $output);
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
  my @docu = create_doc($in, $verbose);
  my $file_header_proc = "module-".lc($docu[0]);
  #$file_header_proc =~ s/[^a-zA-Z]+/-/g;
  $file_header .= "> - [".$docu[0]."](#".$file_header_proc.")\n";
  $file .= $docu[1];
  print "  WrenDoc: built doc => ".basename($in)."\n";
}

open(my $fh, '>>', $outputs) or die 'WrenDoc Err: Could not open file '.$outputs.' $!';
print $fh $file_header.$file; 
close $fh;
print "WrenDoc: Finished\n";
