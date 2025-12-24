import { PUBLIC_ENABLE_PLATEAU_API } from '$env/static/public';
import { error } from '@sveltejs/kit';

export const load = async () => {
	if (PUBLIC_ENABLE_PLATEAU_API === 'true') {
		return;
	}
	error(404, 'Not Found');
};
