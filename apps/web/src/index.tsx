import "./index.css";

/* @refresh reload */
import { Suspense } from "solid-js";
import { render } from "solid-js/web";
import { Route, Router } from "@solidjs/router";

import Layout from "./layout";
import Dashboard from "./pages/dashboard/Dashboard";
import Auth from "./pages/auth/Auth";
import NotFound from "./pages/404/404";

render(
	() => (
		<Router root={Layout}>
			<Route
				path="/"
				component={() => {
					return (
						<Suspense fallback={<div>loading...</div>}>
							<Dashboard />
						</Suspense>
					);
				}}
			/>
			<Route path="/auth" component={Auth} />
			<Route path="*404" component={NotFound} />
		</Router>
	),
	document.getElementById("root") as HTMLElement,
);
