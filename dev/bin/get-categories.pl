use v5.32;

use strict;
use warnings;

use LWP::UserAgent;
use Mojo::DOM;
use Time::HiRes qw( sleep );

sub main {
    my %links = get_links();

    my %classes;
    for my $cat ( sort keys %links ) {
        for my $url ( $links{$cat}->@* ) {
            say $url;
            my $dom   = Mojo::DOM->new( get($url) );
            my @rows = $dom->find('table')->[0]->children('tbody')->[0]->children('tr')->@*;
            for my $row (@rows ) {
                my $td = $row->children('td')->[0];
                push $classes{$cat}->@*, $td->text;
            }
        }
    }

    for my $cat (sort keys %classes) {
        for my $c (sort $classes{$cat}->@*) {
            say qq{("$c", "$cat"),};
        }
    }
}

sub get_links {
    my $dom = Mojo::DOM->new( get( make_url('/docs/installation') ) );
    my $nav = $dom->find("#nav")->[0] // die "Cannot find #nav";

    my %skip = map { $_ => 1 } (
        'Getting Started', 'Core Concepts', 'Customization',
        'Base Styles',     'Official Plugins',
    );

    my %links;
    for my $h5 ( $nav->find('h5')->@* ) {
        my $title = $h5->text;
        next if $skip{$title};

        my $category = clean_category($title);
        my $next      = $h5->following('ul')->[0];
        for my $li ( $next->children('li')->@* ) {
            my $link = $li->children->[0];

            $links{$category} //= [];
            push $links{$category}->@*, make_url( $link->attr('href') );
        }
    }
    return %links;
}

sub get {
    my $url = shift;

    my $ua
        = LWP::UserAgent->new( agent =>
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36'
        );
    my $resp = $ua->get($url);

    my $content = $resp->decoded_content;
    return $content if $resp->is_success;

    my $msg = "Got an error from $url: " . $resp->status_line . "\n";
    if ($content) {
        $msg .= $content;
    }
    die $msg;
}

my $base = 'https://tailwindcss.com';

sub make_url {
    $base . shift;
}

my %short = (
    'Transitions & Animation' => 'animation',
    'Flexbox & Grid'          => 'flex_and_grid',
);

sub clean_category {
    my $name = shift;

    return $short{$name} if $short{$name};
    return lc $name;
}

main();
