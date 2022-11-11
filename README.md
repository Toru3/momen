木綿(momen) is low overhead thread pool library.
木綿(momen) means cotton in Japanese.

# Usage
```rust
fn daxpy(alpha: f64, x: &[f64], y: &mut [f64]) {
    y.iter_mut().zip(x.iter()).for_each(|(y, x)| *y += alpha * *x);
}
let thread_pool = momen::ThreadPoolDyn::new();
let n = thread_pool.max_len();
let mut x = Vec::with_capacity(1000);
let mut y = vec![0f64; 1000];
for i in 0..1000 {
    x.push(i as f64);
}
let chunck_size = (1000 + n - 1) / n;
let mut v = x.chunks(chunck_size).zip(y.chunks_mut(chunck_size)).collect::<Vec<_>>();
let alpha = std::f64::consts::PI;
thread_pool.run(&mut v, &|(x, y)| daxpy(alpha, x, y));
for i in 0..1000 {
    assert_eq!(alpha * x[i], y[i]);
}
```

# benchmark
<div>
<svg viewBox="0 0 1280 720" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" >
<title>Gnuplot</title>
<desc>Produced by GNUPLOT 5.2 patchlevel 2 </desc>
<g id="gnuplot_canvas">
<rect x="0" y="0" width="1280" height="720" fill="none"/>
<defs>
	<circle id='gpDot' r='0.5' stroke-width='0.5'/>
	<path id='gpPt0' stroke-width='0.222' stroke='currentColor' d='M-1,0 h2 M0,-1 v2'/>
	<path id='gpPt1' stroke-width='0.222' stroke='currentColor' d='M-1,-1 L1,1 M1,-1 L-1,1'/>
	<path id='gpPt2' stroke-width='0.222' stroke='currentColor' d='M-1,0 L1,0 M0,-1 L0,1 M-1,-1 L1,1 M-1,1 L1,-1'/>
	<rect id='gpPt3' stroke-width='0.222' stroke='currentColor' x='-1' y='-1' width='2' height='2'/>
	<rect id='gpPt4' stroke-width='0.222' stroke='currentColor' fill='currentColor' x='-1' y='-1' width='2' height='2'/>
	<circle id='gpPt5' stroke-width='0.222' stroke='currentColor' cx='0' cy='0' r='1'/>
	<use xlink:href='#gpPt5' id='gpPt6' fill='currentColor' stroke='none'/>
	<path id='gpPt7' stroke-width='0.222' stroke='currentColor' d='M0,-1.33 L-1.33,0.67 L1.33,0.67 z'/>
	<use xlink:href='#gpPt7' id='gpPt8' fill='currentColor' stroke='none'/>
	<use xlink:href='#gpPt7' id='gpPt9' stroke='currentColor' transform='rotate(180)'/>
	<use xlink:href='#gpPt9' id='gpPt10' fill='currentColor' stroke='none'/>
	<use xlink:href='#gpPt3' id='gpPt11' stroke='currentColor' transform='rotate(45)'/>
	<use xlink:href='#gpPt11' id='gpPt12' fill='currentColor' stroke='none'/>
	<path id='gpPt13' stroke-width='0.222' stroke='currentColor' d='M0,1.330 L1.265,0.411 L0.782,-1.067 L-0.782,-1.076 L-1.265,0.411 z'/>
	<use xlink:href='#gpPt13' id='gpPt14' fill='currentColor' stroke='none'/>
	<filter id='textbox' filterUnits='objectBoundingBox' x='0' y='0' height='1' width='1'>
	  <feFlood flood-color='white' flood-opacity='1' result='bgnd'/>
	  <feComposite in='SourceGraphic' in2='bgnd' operator='atop'/>
	</filter>
	<filter id='greybox' filterUnits='objectBoundingBox' x='0' y='0' height='1' width='1'>
	  <feFlood flood-color='lightgrey' flood-opacity='1' result='grey'/>
	  <feComposite in='SourceGraphic' in2='grey' operator='atop'/>
	</filter>
</defs>
<g fill="none" color="white" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="black" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="gray" stroke="currentColor" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='gray' stroke-dasharray='2,4' class="gridline"  d='M80.2,662.4 L1088.3,662.4  '/></g>
<g fill="none" color="gray" stroke="gray" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,662.4 L89.2,662.4  '/>	<g transform="translate(71.9,666.3)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="end">
		<text><tspan font-family="Helvetica" > 0.1</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,616.6 L84.7,616.6 M80.2,589.8 L84.7,589.8 M80.2,570.8 L84.7,570.8 M80.2,556.1 L84.7,556.1 M80.2,544.1 L84.7,544.1 M80.2,533.9 L84.7,533.9 M80.2,525.1 L84.7,525.1 M80.2,517.3 L84.7,517.3 '/></g>
<g fill="none" color="black" stroke="black" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="gray" stroke="currentColor" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='gray' stroke-dasharray='2,4' class="gridline"  d='M80.2,510.3 L1088.3,510.3  '/></g>
<g fill="none" color="gray" stroke="gray" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,510.3 L89.2,510.3  '/>	<g transform="translate(71.9,514.2)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="end">
		<text><tspan font-family="Helvetica" > 1</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,464.5 L84.7,464.5 M80.2,437.8 L84.7,437.8 M80.2,418.8 L84.7,418.8 M80.2,404.0 L84.7,404.0 M80.2,392.0 L84.7,392.0 M80.2,381.8 L84.7,381.8 M80.2,373.0 L84.7,373.0 M80.2,365.2 L84.7,365.2 '/></g>
<g fill="none" color="black" stroke="black" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="gray" stroke="currentColor" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='gray' stroke-dasharray='2,4' class="gridline"  d='M80.2,358.2 L1088.3,358.2  '/></g>
<g fill="none" color="gray" stroke="gray" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,358.2 L89.2,358.2  '/>	<g transform="translate(71.9,362.1)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="end">
		<text><tspan font-family="Helvetica" > 10</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,312.5 L84.7,312.5 M80.2,285.7 L84.7,285.7 M80.2,266.7 L84.7,266.7 M80.2,252.0 L84.7,252.0 M80.2,239.9 L84.7,239.9 M80.2,229.7 L84.7,229.7 M80.2,220.9 L84.7,220.9 M80.2,213.1 L84.7,213.1 '/></g>
<g fill="none" color="black" stroke="black" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="gray" stroke="currentColor" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='gray' stroke-dasharray='2,4' class="gridline"  d='M80.2,206.2 L1088.3,206.2  '/></g>
<g fill="none" color="gray" stroke="gray" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,206.2 L89.2,206.2  '/>	<g transform="translate(71.9,210.1)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="end">
		<text><tspan font-family="Helvetica" > 100</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,160.4 L84.7,160.4 M80.2,133.6 L84.7,133.6 M80.2,114.6 L84.7,114.6 M80.2,99.9 L84.7,99.9 M80.2,87.8 L84.7,87.8 M80.2,77.7 L84.7,77.7 M80.2,68.8 L84.7,68.8 M80.2,61.1 L84.7,61.1 '/></g>
<g fill="none" color="black" stroke="black" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="gray" stroke="currentColor" stroke-width="0.50" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='gray' stroke-dasharray='2,4' class="gridline"  d='M80.2,54.1 L1088.3,54.1  '/></g>
<g fill="none" color="gray" stroke="gray" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,54.1 L89.2,54.1  '/>	<g transform="translate(71.9,58.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="end">
		<text><tspan font-family="Helvetica" > 1000</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M109.2,662.4 L109.2,657.9 M141.7,662.4 L141.7,657.9 M168.2,662.4 L168.2,657.9 M190.6,662.4 L190.6,657.9 M210.0,662.4 L210.0,657.9 M227.1,662.4 L227.1,657.9 M242.5,662.4 L242.5,653.4  '/>	<g transform="translate(242.5,684.3)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" > 100000</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M343.3,662.4 L343.3,657.9 M402.2,662.4 L402.2,657.9 M444.1,662.4 L444.1,657.9 M476.5,662.4 L476.5,657.9 M503.1,662.4 L503.1,657.9 M525.5,662.4 L525.5,657.9 M544.9,662.4 L544.9,657.9 M562.0,662.4 L562.0,657.9 M577.4,662.4 L577.4,653.4  '/>	<g transform="translate(577.4,684.3)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" > 1x10</tspan><tspan font-family="Helvetica"  font-size="9.6" dy="-6.00px">6</tspan><tspan font-size="12.0" dy="6.00"></tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M678.2,662.4 L678.2,657.9 M737.1,662.4 L737.1,657.9 M779.0,662.4 L779.0,657.9 M811.4,662.4 L811.4,657.9 M837.9,662.4 L837.9,657.9 M860.4,662.4 L860.4,657.9 M879.8,662.4 L879.8,657.9 M896.9,662.4 L896.9,657.9 M912.2,662.4 L912.2,653.4  '/>	<g transform="translate(912.2,684.3)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" > 1x10</tspan><tspan font-family="Helvetica"  font-size="9.6" dy="-6.00px">7</tspan><tspan font-size="12.0" dy="6.00"></tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M1013.0,662.4 L1013.0,657.9 M1072.0,662.4 L1072.0,657.9  '/></g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,54.1 L80.2,662.4 L1088.3,662.4 L1088.3,54.1 L80.2,54.1 Z  '/></g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(16.3,358.3) rotate(270)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" >Average time (µs)</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(584.2,711.3)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" >Input Size (Bytes)</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(584.2,31.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="middle">
		<text><tspan font-family="Helvetica" >copy: Comparison</tspan></text>
	</g>
</g>
	<g id="gnuplot_plot_1" ><title>Rayon</title>
<g fill="none" color="white" stroke="black" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(1163.7,67.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="start">
		<text><tspan font-family="Helvetica" >Rayon</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='rgb(178,  34,  34)'  d='M1113.2,63.1 L1155.4,63.1 M80.2,260.9 L181.0,248.8 L281.8,238.7 L382.6,230.8 L483.4,224.2 L584.3,216.8 L685.1,207.5 L785.9,195.8 L886.7,179.8 L987.5,157.8 L1088.3,128.6  '/></g>
	</g>
	<g id="gnuplot_plot_2" ><title>gnuplot_plot_2</title>
<g fill="none" color="white" stroke="rgb(178,  34,  34)" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<use xlink:href='#gpPt6' transform='translate(80.2,260.9) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(181.0,248.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(281.8,238.7) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(382.6,230.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(483.4,224.2) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(584.3,216.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(685.1,207.5) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(785.9,195.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(886.7,179.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(987.5,157.8) scale(3.38)' color='rgb(178,  34,  34)'/>
	<use xlink:href='#gpPt6' transform='translate(1088.3,128.6) scale(3.38)' color='rgb(178,  34,  34)'/>
</g>
	</g>
	<g id="gnuplot_plot_3" ><title>Rayon chunk</title>
<g fill="none" color="white" stroke="rgb(178,  34,  34)" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(1163.7,85.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="start">
		<text><tspan font-family="Helvetica" >Rayon chunk</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='rgb( 46, 139,  87)'  d='M1113.2,81.1 L1155.4,81.1 M80.2,318.3 L181.0,321.3 L281.8,323.8 L382.6,324.5 L483.4,321.5 L584.3,308.9 L685.1,288.4 L785.9,257.3 L886.7,223.1 L987.5,181.6 L1088.3,140.5  '/></g>
	</g>
	<g id="gnuplot_plot_4" ><title>gnuplot_plot_4</title>
<g fill="none" color="white" stroke="rgb( 46, 139,  87)" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<use xlink:href='#gpPt6' transform='translate(80.2,318.3) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(181.0,321.3) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(281.8,323.8) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(382.6,324.5) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(483.4,321.5) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(584.3,308.9) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(685.1,288.4) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(785.9,257.3) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(886.7,223.1) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(987.5,181.6) scale(3.38)' color='rgb( 46, 139,  87)'/>
	<use xlink:href='#gpPt6' transform='translate(1088.3,140.5) scale(3.38)' color='rgb( 46, 139,  87)'/>
</g>
	</g>
	<g id="gnuplot_plot_5" ><title>Reference</title>
<g fill="none" color="white" stroke="rgb( 46, 139,  87)" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(1163.7,103.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="start">
		<text><tspan font-family="Helvetica" >Reference</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='rgb(  0, 139, 139)'  d='M1113.2,99.1 L1155.4,99.1 M80.2,609.8 L181.0,525.7 L281.8,478.1 L382.6,436.6 L483.4,386.3 L584.3,342.5 L685.1,300.7 L785.9,226.8 L886.7,181.0 L987.5,135.4 L1088.3,89.5  '/></g>
	</g>
	<g id="gnuplot_plot_6" ><title>gnuplot_plot_6</title>
<g fill="none" color="white" stroke="rgb(  0, 139, 139)" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<use xlink:href='#gpPt6' transform='translate(80.2,609.8) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(181.0,525.7) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(281.8,478.1) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(382.6,436.6) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(483.4,386.3) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(584.3,342.5) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(685.1,300.7) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(785.9,226.8) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(886.7,181.0) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(987.5,135.4) scale(3.38)' color='rgb(  0, 139, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(1088.3,89.5) scale(3.38)' color='rgb(  0, 139, 139)'/>
</g>
	</g>
	<g id="gnuplot_plot_7" ><title>ThreadPool</title>
<g fill="none" color="white" stroke="rgb(  0, 139, 139)" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(1163.7,121.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="start">
		<text><tspan font-family="Helvetica" >ThreadPool</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='rgb(255, 215,   0)'  d='M1113.2,117.1 L1155.4,117.1 M80.2,514.8 L181.0,510.6 L281.8,503.2 L382.6,493.7 L483.4,495.7 L584.3,481.7 L685.1,463.4 L785.9,431.1 L886.7,384.5 L987.5,320.6 L1088.3,271.0  '/></g>
	</g>
	<g id="gnuplot_plot_8" ><title>gnuplot_plot_8</title>
<g fill="none" color="white" stroke="rgb(255, 215,   0)" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<use xlink:href='#gpPt6' transform='translate(80.2,514.8) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(181.0,510.6) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(281.8,503.2) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(382.6,493.7) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(483.4,495.7) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(584.3,481.7) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(685.1,463.4) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(785.9,431.1) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(886.7,384.5) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(987.5,320.6) scale(3.38)' color='rgb(255, 215,   0)'/>
	<use xlink:href='#gpPt6' transform='translate(1088.3,271.0) scale(3.38)' color='rgb(255, 215,   0)'/>
</g>
	</g>
	<g id="gnuplot_plot_9" ><title>ThreadPoolDyn</title>
<g fill="none" color="white" stroke="rgb(255, 215,   0)" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<g transform="translate(1163.7,139.0)" stroke="none" fill="black" font-family="Helvetica" font-size="12.00"  text-anchor="start">
		<text><tspan font-family="Helvetica" >ThreadPoolDyn</tspan></text>
	</g>
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='rgb(  0,   0, 139)'  d='M1113.2,135.1 L1155.4,135.1 M80.2,499.9 L181.0,495.5 L281.8,489.4 L382.6,481.6 L483.4,482.4 L584.3,472.9 L685.1,456.7 L785.9,426.1 L886.7,381.1 L987.5,319.4 L1088.3,270.9  '/></g>
	</g>
	<g id="gnuplot_plot_10" ><title>gnuplot_plot_10</title>
<g fill="none" color="white" stroke="rgb(  0,   0, 139)" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<use xlink:href='#gpPt6' transform='translate(80.2,499.9) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(181.0,495.5) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(281.8,489.4) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(382.6,481.6) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(483.4,482.4) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(584.3,472.9) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(685.1,456.7) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(785.9,426.1) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(886.7,381.1) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(987.5,319.4) scale(3.38)' color='rgb(  0,   0, 139)'/>
	<use xlink:href='#gpPt6' transform='translate(1088.3,270.9) scale(3.38)' color='rgb(  0,   0, 139)'/>
</g>
	</g>
<g fill="none" color="white" stroke="rgb(  0,   0, 139)" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="2.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="black" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
	<path stroke='black'  d='M80.2,54.1 L80.2,662.4 L1088.3,662.4 L1088.3,54.1 L80.2,54.1 Z  '/></g>
<g fill="none" color="black" stroke="currentColor" stroke-width="1.00" stroke-linecap="butt" stroke-linejoin="miter">
</g>
</g>
</svg>
</div>

* OS : Ubuntu
* CPU : Ryzen 9 5950X
* MEM : DDR4 3600MHz 128GB
* momen = "0.1.0"
* rayon = "1.5.3"
