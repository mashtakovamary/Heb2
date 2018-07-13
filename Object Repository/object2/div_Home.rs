<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Home</name>
   <tag></tag>
   <elementGuidId>a67d7d5e-5698-4d76-8aa2-a39406671052</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>column main</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>    
        
                                            
                                            Home
                                    
                                    
                        Ways To Shop
                    
                                        
                        Departments
                    
                                                                                
                                            Account Details
                                    
                                                            
                                            Save for Later Lists
                                    
                                    
    


    
    Save for Later Lists



    
        window.authenticationPopup = {&quot;autocomplete&quot;:&quot;off&quot;,&quot;customerRegisterUrl&quot;:&quot;https:\/\/qa.cert.hebtoyou.net\/customer\/account\/create\/&quot;,&quot;customerForgotPasswordUrl&quot;:&quot;https:\/\/qa.cert.hebtoyou.net\/customer\/account\/forgotpassword\/&quot;,&quot;baseUrl&quot;:&quot;https:\/\/qa.cert.hebtoyou.net\/&quot;};
    
    



    








    
    
        
            Create New List
        
        
            
                                
                    
                        List Name
                        
                        
                            Create List                        
                    
                
            
        
        
            // &lt;![CDATA[
            require([
                'jquery',
                'domReady'
            ], function ($, domReady) {
                domReady(function () {
                    $('#wishlist-create-button').on('click', function () {
                        $('#wishlist-create-button').hide();
                        $('#block-create-wishlist-form').show();
                    });

                    /** prevent standard form submitting by pressing enter*/
                    $('#create-wishlist-form').on('keypress', function (event) {
                        if (event.keyCode == 13) {
                            event.preventDefault();
                            $('#create-wishlist-form .action.save').trigger('click');
                        }
                    });
                    /** prevent standard form submitting by pressing enter*/
                });
            });
            // ]]>
        
    
    
    
        
            
                 Actions
                List Name
                Items
            
        
        
                    
                
                    
                    
                
                
                    wishlist test
                    0 items
                
                0 items
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    mnb
                    1 item
                
                1 item
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    678
                    1 item
                
                1 item
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    91011
                    1 item
                
                1 item
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    789
                    0 items
                
                0 items
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    456
                    0 items
                
                0 items
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    test123
                    0 items
                
                0 items
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    test
                    1 item
                
                1 item
                
                    
                        Details                    
                
            
                    
                
                    
                    
                
                
                    Wish List
                    0 items
                
                0 items
                
                    
                        Details                    
                
            
                
    
    
        You have no Save for Later Lists    


    &lt;tr>
        &lt;td class=&quot;action&quot;>
            &lt;button type=&quot;button&quot; title=&quot;Delete List&quot;
                    data-wishlist-id=&quot;&lt;%- data.id %>&quot;
                    class=&quot;action delete&quot;>
            &lt;/button>
        &lt;/td>
        &lt;td>
            &lt;div class=&quot;name&quot;>&lt;%- data.name %>&lt;/div>
            &lt;div class=&quot;visible-mobile&quot;>&lt;%- data.count %>&lt;/div>
        &lt;/td>
        &lt;td class=&quot;visible-desktop&quot;>&lt;%- data.count %>&lt;/td>
        &lt;td>
            &lt;a class=&quot;action&quot; href=&quot;&lt;%- data.url %>&quot;>
                Details            &lt;/a>
        &lt;/td>
    &lt;/tr>


    
    &lt;div id=&quot;map-popup-click-for-price&quot; class=&quot;map-popup&quot;>
        &lt;div class=&quot;popup-header&quot;>
            &lt;strong class=&quot;title&quot; id=&quot;map-popup-heading-price&quot;>&lt;/strong>
        &lt;/div>
        &lt;div class=&quot;popup-content&quot;>
            &lt;div class=&quot;map-info-price&quot; id=&quot;map-popup-content&quot;>
                &lt;div class=&quot;price-box&quot;>
                    &lt;div class=&quot;map-msrp&quot; id=&quot;map-popup-msrp-box&quot;>
                        &lt;span class=&quot;label&quot;>Price&lt;/span>
                        &lt;span class=&quot;old-price map-old-price&quot; id=&quot;map-popup-msrp&quot;>
                            &lt;span class=&quot;price&quot;>&lt;/span>
                        &lt;/span>
                    &lt;/div>
                    &lt;div class=&quot;map-price&quot; id=&quot;map-popup-price-box&quot;>
                        &lt;span class=&quot;label&quot;>Actual Price&lt;/span>
                        &lt;span id=&quot;map-popup-price&quot; class=&quot;actual-price&quot;>&lt;/span>
                    &lt;/div>
                &lt;/div>
                &lt;form action=&quot;&quot; method=&quot;POST&quot; id=&quot;product_addtocart_form_from_popup&quot; class=&quot;map-form-addtocart&quot;>
                    &lt;input type=&quot;hidden&quot; name=&quot;product&quot; class=&quot;product_id&quot; value=&quot;&quot; id=&quot;map-popup-product-id&quot;/>
                    &lt;button type=&quot;button&quot;
                            title=&quot;Add to Cart&quot;
                            class=&quot;action tocart primary&quot;
                            id=&quot;map-popup-button&quot;>
                        &lt;span>Add to Cart&lt;/span>
                    &lt;/button>
                    &lt;div class=&quot;additional-addtocart-box&quot;>
                                            &lt;/div>
                &lt;/form>
            &lt;/div>
            &lt;div class=&quot;map-text&quot; id=&quot;map-popup-text&quot;>
                Our price is lower than the manufacturer&amp;#039;s &amp;quot;minimum advertised price.&amp;quot; As a result, we cannot show you the price in catalog or the product page. &lt;br>&lt;br> You have no obligation to purchase the product once you know the price. You can simply remove the item from your cart.            &lt;/div>
        &lt;/div>
    &lt;/div>
    
    
    &lt;div id=&quot;map-popup-what-this&quot; class=&quot;map-popup&quot;>
        &lt;div class=&quot;popup-header&quot;>
            &lt;strong class=&quot;title&quot; id=&quot;map-popup-heading-what-this&quot;>&lt;/strong>
        &lt;/div>
        &lt;div class=&quot;popup-content&quot;>
            &lt;div class=&quot;map-help-text&quot; id=&quot;map-popup-text-what-this&quot;>
                Our price is lower than the manufacturer&amp;#039;s &amp;quot;minimum advertised price.&amp;quot; As a result, we cannot show you the price in catalog or the product page. &lt;br>&lt;br> You have no obligation to purchase the product once you know the price. You can simply remove the item from your cart.            &lt;/div>
        &lt;/div>
    &lt;/div>
    
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;maincontent&quot;)/div[@class=&quot;columns&quot;]/div[@class=&quot;column main&quot;]</value>
   </webElementProperties>
</WebElementEntity>
