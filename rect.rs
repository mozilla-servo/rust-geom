import num::Num;
import point::Point2D;
import size::Size2D;

struct Rect<T:copy Num> {
    let origin: Point2D<T>;
    let size: Size2D<T>;

    new(origin: Point2D<T>, size: Size2D<T>) {
        self.origin = copy origin;
        self.size = copy size;
    }
}

