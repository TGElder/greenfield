use std::cmp::Reverse;
use std::collections::HashMap;

use rand::{thread_rng, Rng};


const MARKETS: usize = 2048;
const RESOURCES: [Resource; 10] = [
    Resource::Red, Resource::Blue, Resource::Green, Resource::Yellow, Resource::Orange, Resource::Purple, Resource::Pink, Resource::Brown, Resource::Black,  Resource::White
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Resource{
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Pink,
    Brown,
    Black,
    White
}

#[derive(Clone, Copy, Debug)]
struct Product{
    resource: Resource,
    market: usize,
    distance: usize,
}

#[derive(Debug)]
struct Market{
    id: usize,
    demand: Vec<Product>,
    supply: Vec<Product>
}

#[derive(Debug)]
struct Allocation{
    demand: Product,
    supply: Product,
    distance: usize,
}

struct MarketAllocation{
    demand: HashMap<Resource, Vec<Product>>,
    supply: HashMap<Resource, Vec<Product>>,
}

fn main() {
    println!("Generating markets");
    let markets = random_markets();
    let mut distances = Box::new([[0; MARKETS]; MARKETS]);
    for i in 0..MARKETS {
        for j in i+1..MARKETS {
            distances[i][j] = random_inter_distance();
        }
    }
    let allocations = allocate(&markets, &distances);
    println!("{}", allocations.len());
}


fn allocate(markets: &[Market], distances: &[[usize; MARKETS]; MARKETS]) -> Vec<Allocation> {
    let mut out = vec![];
    let mut demand_allocations = markets.iter().map(|market| allocation(&market.demand)).collect::<Vec<_>>();
    let mut supply_allocations = markets.iter().map(|market| allocation(&market.supply)).collect::<Vec<_>>();

    let mut pairs: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..markets.len() {
        for j in 0..markets.len() {
            pairs.push((i, j, distances[i][j]));
        }
    }

    pairs.sort_by_key(|(_, _, distance)| *distance);


    for (i, j, distance) in pairs.iter() {
        for resource in RESOURCES {
            let Some(supply) = supply_allocations[*i].get_mut(&resource) else {continue;};
            let Some(demand) = demand_allocations[*j].get_mut(&resource) else {continue;};

            while !supply.is_empty() && !demand.is_empty() {
                let allocation = 
                    Allocation{
                        supply: supply.pop().unwrap(),
                        demand: demand.pop().unwrap(),
                        distance: *distance
                    }
                ;
                out.push(allocation);
            }
        }
    }

    out
}

fn random_markets() -> Vec<Market> {
    (0..MARKETS).map(|i| random_market(i)).collect()
}

fn random_market(id: usize) -> Market {

    Market { id, demand: random_products(id), supply: random_products(id) }

}

fn random_products(market: usize) -> Vec<Product> {
    let mut out = vec![];
    for resource in RESOURCES {
        for _ in 0..random_amount() {
            out.push(Product{
                resource,
                market,
                distance: random_intra_distance(),
            });
        }
    }
    out
}

fn random_amount() -> usize {
    thread_rng().gen_range(0..10usize).pow(2)
}

fn random_intra_distance() -> usize {
    thread_rng().gen_range(0..1024)
}

fn random_inter_distance() -> usize {
    thread_rng().gen_range(0..1048576)
}


fn allocation(products: &[Product]) -> HashMap<Resource, Vec<Product>>{
    let mut out: HashMap<Resource, Vec<Product>> = HashMap::with_capacity(RESOURCES.len());

    for product in products {
        out.entry(product.resource).or_default().push(*product);
    }

    for list in out.values_mut() {
        list.sort_by_key(|product| Reverse(product.distance));
    }

    out
    
}