import {AdUser} from "../models/groups";
import nunjucks from "nunjucks";
import {Banner} from "../models/exercise";

export const prepareBannerForDeploymentUser = (
	banner: Banner,
	exerciseName: string,
	deploymentName: string,
) => ({
	...banner,
	content: nunjucks.renderString(banner.content, {
		exerciseName,
		deploymentName,
		participantUsername: 'hue',
	}),
});