param (
    $registry = "registry.meisterlala.dev",
    $imageName = "advent-of-code-2025",
    $tag,
    $arch = "linux/amd64,linux/arm64"
)

# Set tag to 'latest' and current git commit hash if not provided
$tagParams = @()
if ($tag)
{
    $tagParams += "-t", "$registry/${imageName}:$tag"
} else
{
    $tagParams += "-t", "$registry/${imageName}:latest"
    $gitCommit = git rev-parse --short HEAD
    if ($gitCommit)
    {
        $tagParams += "-t", "$registry/${imageName}:$gitCommit"
    }
}


# Set up builder
docker buildx create --use --driver docker-container

# Build and push multi-arch image
docker buildx build --platform $arch @tagParams --push .