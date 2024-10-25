
#[derive(Debug)]
struct BlogPost {
    content: String
}

impl BlogPost {
    // Esto es una associated fuction 
    fn from_string(content: String) -> Self {

        let blog_post = BlogPost { content };
        
        return blog_post;
    }
}
fn main() {
    let post_content = String::from("This is a blog post about Rust.");
    
   // creamos una associated function en main
    let blog_post = BlogPost::from_string(post_content);
    
    println!("Blog post content: {:#?}", blog_post.content);
}
