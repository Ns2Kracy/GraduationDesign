/* @refresh reload */
import { Suspense } from "solid-js";
import { render } from "solid-js/web";
import { Route, Router } from "@solidjs/router";

import "./index.css";
import { Layout } from "./layout";
import { Dashboard } from "./pages/dashboard/Dashboard";
import { NotFound } from "./pages/404/404";
import { Auth } from "./pages/auth/Auth";

render(
	() => (
		<Router root={Layout}>
			<Route
				path="/dashboard"
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
