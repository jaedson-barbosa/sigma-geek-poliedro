const N: usize = 14;
const R: usize = 4;
const TOLERANCE: f64 = 0.001;
static mut COUNTER: i32 = 0;

#[derive(Clone, Debug, Copy, Default)]
struct Point(f64, f64, f64);

fn cross_product(left: Point, right: Point) -> Point {
    let tmp0 = left.1 * right.2 - left.2 * right.1;
    let tmp1 = left.2 * right.0 - left.0 * right.2;
    let tmp2 = left.0 * right.1 - left.1 * right.0;
    return Point(tmp0, tmp1, tmp2);
}

fn dot_product(left: Point, right: Point) -> f64 {
    return left.0 * right.0 + left.1 * right.1 + left.2 * right.2;
}

struct Plane3D {
    independent_term: f64,
    normal: Point,
}

fn vector(p1: Point, p2: Point) -> Point {
    Point(p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2)
}

impl Plane3D {
    fn create(p1: Point, p2: Point, p3: Point) -> Self {
        let normal = cross_product(vector(p1, p2), vector(p1, p3));
        let magnitude = dot_product(normal, normal).sqrt();
        if magnitude < TOLERANCE {
            panic!("Specified points do not define a valid plane.");
        }
        let independent_term = -(dot_product(normal, p1));
        Plane3D {
            normal,
            independent_term,
        }
    }

    fn contains(&self, p: Point) -> bool {
        (dot_product(self.normal, p) + self.independent_term).abs() < TOLERANCE
    }
}

fn analyse_points(points: &[Point; R]) {
    let plane = Plane3D::create(points[0], points[1], points[2]);
    let valid = !plane.contains(points[3]);
    if valid {
        println!("{points:?}");
        unsafe {
            COUNTER += 1;
        }
    }
}

fn print_combination(arr: [Point; N]) {
    // A temporary array to store all combination one by one
    let mut data: [Point; R] = [Point::default(); R];

    // Print all combination using temporary array 'data[]'
    combination_util(&arr, &mut data, 0, N - 1, 0);
}

/* arr[]  ---> Input Array
data[] ---> Temporary array to store current combination
start & end ---> Starting and Ending indexes in arr[]
index  ---> Current index in data[]
r ---> Size of a combination to be printed */
fn combination_util(
    arr: &[Point; N],
    data: &mut [Point; R],
    start: usize,
    end: usize,
    index: usize,
) {
    // Current combination is ready to be printed, print it
    if index == R {
        analyse_points(data);
        return;
    }

    // replace index with all possible elements. The condition
    // "end-i+1 >= r-index" makes sure that including one element
    // at index will make a combination with remaining elements
    // at remaining positions
    let mut i = start;
    while i <= end && end - i + 1 >= R - index {
        data[index] = arr[i];
        combination_util(arr, data, i + 1, end, index + 1);
        i += 1;
    }
}

fn main() {
    let p: f64 = 3.0;
    let n: f64 = -3.0;
    let z: f64 = 0.0;
    let vertices: [Point; 6] = [
        Point(p, p, z),
        Point(p, n, z),
        Point(n, p, z),
        Point(n, n, z),
        Point(z, z, p),
        Point(z, z, n),
    ];
    let centro = |i, j, k| {
        let a: Point = vertices[i];
        let b: Point = vertices[j];
        let c: Point = vertices[k];
        let mean = |a, b, c| (a + b + c) / 3.0;
        return Point(
            mean(a.0, b.0, c.0),
            mean(a.1, b.1, c.1),
            mean(a.2, b.2, c.2),
        );
    };
    let centros = [
        centro(0, 1, 4),
        centro(0, 2, 4),
        centro(1, 3, 4),
        centro(2, 3, 4),
        centro(0, 1, 5),
        centro(0, 2, 5),
        centro(1, 3, 5),
        centro(2, 3, 5),
    ];

    let mut pontos = [Point::default(); N];
    let (left, right) = pontos.split_at_mut(6);
    left.clone_from_slice(&vertices);
    right.clone_from_slice(&centros);

    print_combination(pontos);
    unsafe {
        println!("Total: {COUNTER}");
    }
}
