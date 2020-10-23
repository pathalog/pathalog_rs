use serde::export::Formatter;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::io::SeekFrom::End;
#[macro_use]
extern crate maplit;

type Truth = Option<bool>;

///A Path is only distinguished by its length
type Path = u8;

///A POSITION is the index of location of a point on a single dimensional path
/// using a u8 for now because we are only using toy paths
type POSITION = u8;

///A point refers to a particular position on a particular path (as indexed by the local universe);
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point(POSITION, POSITION);

///An Entanglement ensure that multiple otherwise disjoint Points on various Paths are now
/// entangled such that they must always have the same value and lifespan
/// An Entanglement can be viewed as its own path through its entangled points
type Entanglement = Vec<(POSITION,POSITION)>;

#[derive(Default, Debug)]
struct Entanglements(Vec<Entanglement>);

#[derive(Default)]
struct Paths {
    paths: Vec<Path>,
}

impl Debug for Paths {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "");
        for (i, path) in &mut self.paths.iter().enumerate() {
            writeln!(f, "Path {},Length {}", i, path);
        }
        Ok(())
    }
}

impl Paths {
    fn join(mut self, mut paths: Paths) -> Self {
        self.paths.append(&mut paths.paths);
        self
    }
}

#[derive(Default, Debug, Clone)]
struct Observations(Vec<Point>);

#[derive(Default, Debug, Clone)]
struct Inputs {
    inputs: HashMap<Point, Truth>,
}

///This is a different take on vertex and edge grapth notation
/// Instead of denoting all vertexes and edges individually, Paths
/// consist of multiple vertexes connected linearly, and additional
/// edges are added by entangling points on a path.
/// This is fudamentally equivalent to building up a graph with vertexes
/// and edges, but encourages one to think in a very different manner
#[derive(Default, Debug)]
struct Universe {
    paths: Paths,
    entanglements: Entanglements,
    inputs: Inputs,
    observations: Observations,
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        panic!();
    }
}

impl Universe {
    fn get_inputs(&mut self, point: &Point) -> Vec<Point> {
        for (k, v) in &self.inputs.inputs.clone() {
            if k == point {
                println!("found input: {:?}:{:?}", k, v);
                //self.inputs.inputs.pop();
            }
        }

        vec![]
    }

    fn path_inputs(&mut self) -> Vec<Point> {
        //println!("path inputs: ");
        for (path_idx, path_len) in self.paths.paths.iter().enumerate() {
            self.inputs.inputs.insert(Point(path_idx as u8,*path_len),None);
            for i in 0..*path_len {
                self.inputs.inputs.insert(Point(path_idx as u8,*path_len),None);
          //      println!("\t{}:{}", path_idx , *path_len);
            }
        }
        vec!()
    }

    fn path_outputs(&mut self) -> Vec<Point> {
        println!("path outputs: ");

        for (path_idx, path_len) in self.paths.paths.iter().enumerate() {
            self.observations.0.push(Point(path_idx as u8,*path_len));
          //  println!("\t{}:{}", path_idx, path_len);
        }
        vec!()
    }

    fn entanglements(&mut self) -> Vec<Point> {
        //println!("entanglements: ");
        for entangled_path in &self.entanglements.0 {
            for point in entangled_path {
            //    print!("=> {}:{}\t",point.0,point.1);
            }
            println!();
//            self.inputs.inputs
//            println!("\t{}:{}", path_idx + 1, path_len+1);
        }
        vec!()
    }

}

fn main() {
    // let mut paths1:Paths = Paths(vec!(Path(1),Path(4),Path(6)));
    // let mut paths2:Paths = Paths(vec!(Path(2),Path(3),Path(4)));
    // let mut entanglements:Vec<Entanglement> = vec!();
    //
    // paths1.0.append(&mut paths2.0);
    //
    let mut not = Universe {
        paths: Paths {
            paths: vec![3,2,3],
        },
        entanglements: Entanglements(vec![vec![(0, 0), (0, 1)]]), //,vec![(1, 2), (1, 1)]]
        inputs: Inputs{inputs:hashmap!()},
        observations: Observations(vec![]),
    };
    //dbg!(&u);

    // for (point, truth) in u.observations.0.clone() {
    //     println!("obs: {:?}, {:?}", point, truth);
    // }
    //
    // for (point, truth) in u.observations.0.clone() {
    //     u.get_inputs(&point);
    // }

    not.path_inputs();
    not.path_outputs();
    not.entanglements();

    println!("{:?}", not.paths);
    println!("{:?}", not.inputs);
    println!("{:?}", not.observations);
    println!("{:?}", not.entanglements);

    //walk the paths for outputs, and see which ones are entangled
    //get list of empty outputs
}
