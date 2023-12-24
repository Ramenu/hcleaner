

pub fn convert_bytes_to_appropriate_unit(n : u64) -> (f64, String)
{
    if n < 1024 {
        return (n as f64, "B".to_string());
    }

    let mut n = n as f64 / 1024.0;
    let mut unit = "KiB";

    if n > 1024.0 {
        n /= 1024.0;
        unit = "MiB";

        if n > 1024.0 {
            n /= 1024.0;
            unit = "GiB";

            if n > 1024.0 {
                n /= 1024.0;
                unit = "TiB";
            }
        }
    }

    (n, unit.to_string())
}