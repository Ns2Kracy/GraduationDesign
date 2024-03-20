import { RouteSectionProps } from "@solidjs/router";
import { Component } from "solid-js";

export const Layout: Component<RouteSectionProps> = (props) => {
	return (
		<div>
			<h1>Layout</h1>
			{props.children}
		</div>
	);
};

export default Layout;
