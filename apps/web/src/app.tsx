// @refresh reload
import { Suspense } from "solid-js";
import { Meta, MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start";
import "./app.css";

export default function App() {
	return (
		<Router
			root={(props) => (
				<MetaProvider>
					<Title>农产品安全质量管理系统</Title>
					<Meta name="description" content="农产品安全质量管理系统" />
					<Meta name="charset" content="UTF-8" />
					<Suspense>{props.children}</Suspense>
				</MetaProvider>
			)}
		>
			<FileRoutes />
		</Router>
	);
}
