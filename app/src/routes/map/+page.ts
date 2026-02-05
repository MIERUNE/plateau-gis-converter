import { PUBLIC_ENABLE_PLATEAU_API } from '$env/static/public';
import { error } from '@sveltejs/kit';

export const prerender = false;

export const load = () => {
	if (PUBLIC_ENABLE_PLATEAU_API !== 'true') {
		error(404, 'Not Found');
	}
};
