#!/usr/bin/perl
use strict;
use Data::Dumper;

$Data::Dumper::Indent = 0;
$Data::Dumper::Terse  = 1;

my $factor = $ENV{FACTOR} || 1;
my $rounds = $ENV{ROUNDS} || 1;
my $i      = 0;
my @nums   = map { chomp $_; $_ ne "" ? [ $i++, $_ * $factor ] : () } (<>);

my ( $next, $idx );
print "start: ", Dumper( \@nums ), "\n" if $ENV{DEBUG} >= 1;
for my $z ( 0 .. ( $rounds - 1 ) ) {
    $next = 0;
    $idx  = 0;
    while ( $next < @nums ) {
        my $t = $nums[$idx];

        if ( $t->[0] != $next ) {
            $idx++;
            next;
        }

        splice @nums, $idx, 1;
        my $nidx = ( $idx + $t->[1] ) % @nums;
        splice @nums, $nidx, 0, $t;

        $next++;
        $idx = 0;
    }
    die "didn't shuffle all numbers" if $next != @nums;
    print "round $z: ", Dumper( \@nums ), "\n" if $ENV{DEBUG} >= 1;
}

my $zero;
for $idx ( 0 .. $#nums ) {
    if ( $nums[$idx]->[1] == 0 ) {
        $zero = $idx;
    }
}
die "couldn't find zero" unless defined($zero);
print "zero at $zero\n";

my @final_idx = map { ( $_ + $zero ) % @nums } ( 1000, 2000, 3000 );
my @output    = map { $nums[$_]->[1] } @final_idx;
print "@final_idx -> @output = ", $output[0] + $output[1] + $output[2], "\n";
